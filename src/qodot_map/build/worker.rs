#![allow(clippy::transmute_ptr_to_ptr)] // Suppress gdnative clippy warnings

use std::collections::HashMap;

use crate::texture_loader::TextureInfo;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
    FromVariant, GodotString, Instance, Map, Material, NativeClass, Node, Spatial, Variant
};
use quarchitect::scene_tree::SceneTreeNode;

enum BuildCommand {
    Tick,
}

enum BuildMessage {
    Tick,
    Complete,
}

enum FlatSceneTree<'a> {
    Node(&'a quarchitect::scene_tree::SceneTreeNode),
    PushParent,
    PopParent,
}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct QodotBuildWorker {
    tick_tx: Option<std::sync::mpsc::Sender<BuildCommand>>,
    build_rx: Option<std::sync::mpsc::Receiver<BuildMessage>>,
}

pub struct Config {
    map_file: GodotString,
    texture_info: HashMap<String, TextureInfo>,
    texture_blacklist: quarchitect::TextureBlacklist,
    default_material: Variant,
    default_spatial_material_texture_param: i32,
    default_shader_material_texture_param: GodotString,
    quarchitect_forge_game_data: quarchitect::game_data::forge::GameData,
    quarchitect_game_data: quarchitect::game_data::GameData,
    chunk_size: i32,
}

impl Config {
    pub fn new(
        quarchitect_forge_game_data: quarchitect::game_data::forge::GameData,
        quarchitect_game_data: quarchitect::game_data::GameData,
        map_file: GodotString,
        texture_info: HashMap<String, TextureInfo>,
        texture_blacklist: quarchitect::TextureBlacklist,
        default_material: Variant,
        default_spatial_material_texture_param: i32,
        default_shader_material_texture_param: GodotString,
        chunk_size: i32,
    ) -> Config {
        godot_print!("TODO-2: Refactor to store default material + params in an enum");

        Config {
            quarchitect_forge_game_data,
            quarchitect_game_data,
            map_file,
            texture_info,
            texture_blacklist,
            default_material,
            default_spatial_material_texture_param,
            default_shader_material_texture_param,
            chunk_size,
        }
    }
}

#[methods]
impl QodotBuildWorker {
    // Overrides
    fn _init(_owner: Spatial) -> Self {
        let tick_tx = None;
        let build_rx = None;
        QodotBuildWorker { tick_tx, build_rx }
    }

    #[export]
    fn _enter_tree(&self, owner: Spatial) {
        let main_loop = gdnative::Engine::godot_singleton().get_main_loop();
        assert!(main_loop.is_some());

        unsafe {
            match main_loop.unwrap().connect(
                "idle_frame".into(),
                owner.cast::<gdnative::Object>(),
                "idle_frame".into(),
                gdnative::VariantArray::new(),
                0,
            ) {
                Ok(()) => (),
                Err(err) => {
                    godot_error!("Error connecting idle signal: {:?}", err);
                }
            };
        }
    }

    #[export]
    fn _exit_tree(&self, owner: Spatial) {
        let main_loop = gdnative::Engine::godot_singleton().get_main_loop();
        assert!(main_loop.is_some());

        unsafe {
            main_loop.unwrap().disconnect(
                "idle_frame".into(),
                owner.cast::<gdnative::Object>(),
                "idle_frame".into(),
            );
        }
    }

    #[export]
    fn idle_frame(&mut self, mut owner: Spatial) {
        let mut done = false;

        if let (Some(tick_tx), Some(build_rx)) = (&self.tick_tx, &self.build_rx) {
            loop {
                match build_rx.try_recv() {
                    Ok(message) => match message {
                        BuildMessage::Tick => match tick_tx.send(BuildCommand::Tick) {
                            Ok(()) => (),
                            Err(err) => {
                                godot_error!("Failed to send command to build thread: {:?}", err);
                                done = true;
                                break;
                            }
                        },
                        BuildMessage::Complete => {
                            godot_print!("Build complete");
                            done = true;
                            break;
                        }
                    },
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        godot_print!("Message queue disconnected");
                        done = true;
                        break;
                    }
                }
            }
        }

        if done {
            self.tick_tx = None;
            self.build_rx = None;
            unsafe {
                owner.queue_free();
            }
        }
    }

    pub fn build(&mut self, owner: Spatial, config: Config) {
        let (tick_tx, tick_rx) = std::sync::mpsc::channel();
        self.tick_tx = Some(tick_tx);

        let (build_tx, build_rx) = std::sync::mpsc::channel();
        self.build_rx = Some(build_rx);

        let map_file = gdnative::ProjectSettings::godot_singleton().globalize_path(config.map_file);
        let map_file = map_file.to_string();

        let quarchitect_game_data = config.quarchitect_game_data;
        let quarchitect_forge_game_data = config.quarchitect_forge_game_data;

        let default_material = config.default_material;
        let default_spatial_material_texture_param = config.default_spatial_material_texture_param;
        let default_shader_material_texture_param = config.default_shader_material_texture_param;

        godot_print!("Building texture data");
        let (quarchitect_texture_info, gdnative_texture_info) = config
            .texture_info
            .into_iter()
            .fold((HashMap::new(), HashMap::new()), |mut acc, (key, value)| {
                acc.0.insert(key.to_string(), value.quarchitect_data);

                let extra: HashMap<String, Variant> = value
                    .gdnative_extra
                    .into_iter()
                    .map(|(k, v)| (k, Variant::from_object(&v)))
                    .collect();

                acc.1
                    .insert(key, (Variant::from_object(&value.gdnative_data), extra));
                acc
            });
        let quarchitect_texture_info = quarchitect::TextureInfo(quarchitect_texture_info);

        let texture_blacklist = config.texture_blacklist;
        let chunk_size = config.chunk_size;

        let owner = Variant::from_object(&owner);
        std::thread::spawn(move || {
            let tick_rx = tick_rx;
            let build_tx = build_tx;

            let owner = Instance::<crate::QodotMap>::from_variant(&owner).unwrap();
            let (owner, script) = owner.decouple();

            println!("Building map");

            let config: quarchitect::Config = quarchitect::Config::new(
                &map_file,
                quarchitect_texture_info,
                texture_blacklist,
                quarchitect_forge_game_data,
                quarchitect_game_data,
            );
            let scene_tree = quarchitect::run(config);
            let scene_tree = match scene_tree {
                Ok(scene_tree) => scene_tree,
                Err(err) => {
                    eprintln!("Build error: {}", err);
                    return;
                }
            };

            println!("Populating scene tree");
            script
                .map(|script| {
                    let scene_tree: Vec<FlatSceneTree> = scene_tree.iter().flat_map(flatten_scene_tree_node).collect();
                    let mut scene_tree_iter = scene_tree.into_iter();
                    let mut current_node: Option<Node> = None;
                    let mut parent_stack: Vec<Option<Node>>;
                    unsafe {
                        parent_stack = vec![owner.cast::<Node>()];
                    }

                    loop {
                        let mut done = false;
                        for _i in 0..chunk_size {
                            if let Some(scene_tree) = scene_tree_iter.next() {
                                let scene_tree = match scene_tree {
                                    FlatSceneTree::Node(node) => node,
                                    FlatSceneTree::PushParent => {
                                        parent_stack.push(current_node);
                                        continue;
                                    }
                                    FlatSceneTree::PopParent => {
                                        parent_stack.pop();
                                        continue;
                                    }
                                };

                                match &scene_tree.data {
                                    quarchitect::scene_tree::SceneTreeType::Actor(
                                        actor,
                                        _children,
                                    ) => {
                                        current_node = super::scene_tree::spawn_scene_tree_actor(
                                            script.inverse_scale_factor,
                                            owner,
                                            &parent_stack[parent_stack.len() - 1],
                                            scene_tree,
                                            actor,
                                        );
                                    }
                                    quarchitect::scene_tree::SceneTreeType::VisualGeometry(
                                        visual_geometry,
                                    ) => {
                                        let mesh_instance =
                                            super::visual_geometry::spawn_mesh_instance(
                                                owner,
                                                &parent_stack[parent_stack.len() - 1],
                                                visual_geometry,
                                            );

                                        match build_tx.send(BuildMessage::Tick) {
                                            Ok(_) => (),
                                            Err(err) => {
                                                eprintln!(
                                                    "Error sending message to main thread: {:?}",
                                                    err
                                                );
                                                break;
                                            }
                                        }

                                        match tick_rx.recv() {
                                            Ok(_delta) => (),
                                            Err(_err) => break,
                                        }

                                        match build_tx.send(BuildMessage::Tick) {
                                            Ok(_) => (),
                                            Err(err) => {
                                                eprintln!(
                                                    "Error sending message to main thread: {:?}",
                                                    err
                                                );
                                                break;
                                            }
                                        }

                                        match tick_rx.recv() {
                                            Ok(_delta) => (),
                                            Err(_err) => break,
                                        }

                                        super::visual_geometry::populate_mesh_geometry(
                                            visual_geometry,
                                            mesh_instance,
                                            scene_tree.origin,
                                            script.inverse_scale_factor,
                                        );

                                        super::visual_geometry::populate_mesh_materials(
                                            &gdnative_texture_info,
                                            default_material.try_to_object::<Material>(),
                                            default_spatial_material_texture_param,
                                            &default_shader_material_texture_param,
                                            visual_geometry,
                                            mesh_instance,
                                        );
                                    }
                                    quarchitect::scene_tree::SceneTreeType::CollisionGeometry(
                                        collision_geometry,
                                    ) => {
                                        super::collision_geometry::spawn_collision_geometry(
                                            script.inverse_scale_factor,
                                            owner,
                                            &parent_stack[parent_stack.len() - 1],
                                            collision_geometry,
                                            scene_tree.origin,
                                        );
                                    }
                                }
                            } else {
                                done = true;
                                break;
                            }
                        }

                        if done {
                            break;
                        }

                        match build_tx.send(BuildMessage::Tick) {
                            Ok(_) => (),
                            Err(err) => {
                                eprintln!("Error sending message to main thread: {:?}", err);
                                break;
                            }
                        }

                        match tick_rx.recv() {
                            Ok(_delta) => (),
                            Err(_err) => break,
                        }
                    }
                })
                .unwrap_or_else(|err| eprintln!("Error populating scene tree: {:?}", err));

            match build_tx.send(BuildMessage::Complete) {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("Error sending message to main thread: {:?}", err);
                }
            }
        });
    }
}

fn flatten_scene_tree_node(node: &SceneTreeNode) -> Vec<FlatSceneTree> {
    let mut scene_tree: Vec<FlatSceneTree> = Vec::new();

    scene_tree.push(FlatSceneTree::Node(node));
                        
    if let quarchitect::scene_tree::SceneTreeType::Actor(
        _actor,
        children,
    ) = &node.data
    {
        println!("Processing {:?} children...", children.len());
        scene_tree.push(FlatSceneTree::PushParent);
        for child in children {
            println!("Child...");
            let mut child_scene_tree = flatten_scene_tree_node(child);
            scene_tree.append(&mut child_scene_tree);
        }
        scene_tree.push(FlatSceneTree::PopParent);
    }

    scene_tree
}
