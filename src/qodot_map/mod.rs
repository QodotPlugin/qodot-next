#![allow(clippy::transmute_ptr_to_ptr)] // Suppress gdnative clippy warnings

use gdnative::{
    godot_error, godot_print,
    user_data::{LocalCellData, RwLockData},
    FromVariant, GodotString, Instance, Map, MapMut, NativeClass, Node, Spatial, StringArray,
    Variant,
};
use std::collections::HashMap;

use crate::{
    game_data::{DefaultMaterialType, QodotGameData, QodotMaterialData},
    map::QuakeMap,
    texture_loader,
};

mod build;
mod gdn;

pub use build::worker::QodotBuildWorker;
use texture_loader::PaletteType;

const CHILD_META: &str = "qodot_map_child";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MapType {
    Resource,
    File,
}

impl Into<i32> for MapType {
    fn into(self) -> i32 {
        match self {
            MapType::Resource => 0,
            MapType::File => 1,
        }
    }
}

impl From<i32> for MapType {
    fn from(val: i32) -> MapType {
        match val {
            0 => MapType::Resource,
            1 => MapType::File,
            _ => panic!("Invalid map type"),
        }
    }
}

#[derive(NativeClass)]
#[inherit(Spatial)]
#[user_data(RwLockData<QodotMap>)]
#[register_with(gdn::registration::register_qodot_map)]
pub struct QodotMap {
    forge_game_data: Variant,
    qodot_game_data: Variant,

    map_type: MapType,
    map_resource: Variant,
    automatic_rebuild: bool,
    map_revision: Option<i32>,
    map_file: GodotString,

    wad_palette_type: PaletteType,

    texture_type: crate::texture_loader::TextureType,
    brush_texture_blacklist: StringArray,
    plane_texture_blacklist: StringArray,

    default_material_data: Variant,

    inverse_scale_factor: f32,
    chunk_size: i32,
}

impl QodotMap {
    pub fn _init(_owner: Spatial) -> Self {
        let forge_game_data = Variant::new();
        let qodot_game_data = Variant::new();

        let map_type = MapType::Resource;
        let automatic_rebuild = true;
        let map_file = GodotString::new();
        let map_resource = Variant::new();
        let map_revision = None;

        let texture_type = texture_loader::TextureType::TextureResources("res://".into());

        let wad_palette_type = PaletteType::Resource(Variant::new());

        let mut brush_texture_blacklist = StringArray::new();
        brush_texture_blacklist.push(&GodotString::from_str(&"special/clip"));

        let mut plane_texture_blacklist = StringArray::new();
        plane_texture_blacklist.push(&GodotString::from_str(&"special/skip"));
        plane_texture_blacklist.push(&GodotString::from_str(&"__TB_empty"));

        let default_material_data = Variant::new();

        let inverse_scale_factor = 16.0;
        let chunk_size = 64;

        QodotMap {
            forge_game_data,
            qodot_game_data,

            map_type,
            map_resource,
            automatic_rebuild,
            map_revision,
            map_file,

            texture_type,

            wad_palette_type,

            brush_texture_blacklist,
            plane_texture_blacklist,

            default_material_data,

            inverse_scale_factor,
            chunk_size,
        }
    }

    // Business Logic
    fn build(&mut self, mut owner: Spatial) {
        godot_print!("Clearing entities");
        self.clear_entities(owner);

        godot_print!("Getting map file path");
        let map_file = match self.get_map_path(owner) {
            Ok(map_file) => map_file,
            Err(err) => {
                godot_error!("Failed to get map file path: {}", err);
                return;
            }
        };

        godot_print!("Getting quarchitect forge game data");
        let quarchitect_forge_game_data = match self.get_quarchitect_forge_game_data(owner) {
            Ok(quarchitect_forge_game_data) => quarchitect_forge_game_data,
            Err(err) => {
                godot_error!("Failed to load quarchitect forge game data: {}", err);
                return;
            }
        };

        godot_print!("Getting quarchitect game data");
        let quarchitect_game_data = match self.get_quarchitect_game_data(owner) {
            Ok(quarchitect_game_data) => quarchitect_game_data,
            Err(err) => {
                godot_error!("Failed to load game data: {}", err);
                return;
            }
        };

        godot_print!("Getting texture info");
        let texture_info =
            match texture_loader::load_textures(&self.texture_type, &self.wad_palette_type) {
                Ok(texture_info) => texture_info,
                Err(_) => HashMap::new(),
            };

        godot_print!("Assembling texture blacklist");
        let texture_blacklist = self.get_texture_blacklist();

        godot_print!("Fetching default material");
        let default_material_data =
            Instance::<QodotMaterialData>::from_variant(&self.default_material_data);

        let mut default_material = Variant::new();
        let mut default_spatial_material_texture_param = 0;
        let mut default_shader_material_texture_param = "albedo".into();

        if let Ok(default_material_data) = default_material_data {
            let (base, script) = default_material_data.decouple();

            match script.map(|script| {
                default_material = match script.material_type {
                    DefaultMaterialType::Spatial => script.spatial_material.clone(),
                    DefaultMaterialType::Shader => script.shader_material.clone(),
                };

                default_spatial_material_texture_param =
                    script.get_base_texture_spatial_param(base.clone());

                default_shader_material_texture_param =
                    script.get_base_texture_shader_param(base.clone()).clone();
            }) {
                Ok(()) => (),
                Err(err) => {
                    godot_error!("Error fetching default material data: {:?}", err);
                    return;
                }
            };
        }

        godot_print!("Spawning build worker");
        let build_worker = Instance::<build::worker::QodotBuildWorker>::new();
        let (mut base, script) = build_worker.decouple();
        unsafe {
            (&mut base).set_meta(CHILD_META.into(), Variant::from_bool(true));
            owner.add_child(base.cast::<Node>(), true);
        }

        godot_print!("Running build worker");
        match script.map_mut(|script| {
            script.build(
                owner,
                build::worker::Config::new(
                    quarchitect_forge_game_data,
                    quarchitect_game_data,
                    map_file,
                    texture_info,
                    texture_blacklist,
                    default_material,
                    default_spatial_material_texture_param,
                    default_shader_material_texture_param,
                    self.chunk_size,
                ),
            )
        }) {
            Ok(_) => (),
            Err(err) => {
                godot_error!("Error running build worker: {:?}", err);
            }
        }
    }

    fn clear_entities(&self, mut owner: Spatial) {
        unsafe {
            for i in (0..owner.get_child_count()).rev() {
                if let Some(mut child) = owner.get_child(i) {
                    if child.has_meta(CHILD_META.into()) {
                        owner.remove_child(Some(child));
                        child.queue_free();
                    }
                }
            }
        }
    }

    fn get_map_path(&mut self, owner: Spatial) -> Result<GodotString, String> {
        match self.map_type {
            MapType::Resource => match self.get_map_resource(owner) {
                Some(resource) => {
                    let quake_map: Instance<QuakeMap> = Instance::try_from_base(resource).unwrap();
                    let (base, _script) = quake_map.decouple();
                    let path = (&base).get_path();
                    let new_map_revision;
                    unsafe {
                        new_map_revision = base.get("revision".into()).try_to_i64();
                    }
                    match new_map_revision {
                        Some(new_map_revision) => {
                            self.set_map_revision(owner, Some(new_map_revision as i32))
                        }
                        None => {
                            return Err("Failed to set new map revision".into());
                        }
                    }
                    Ok(path)
                }
                None => Err("No map resource".into()),
            },
            MapType::File => Ok(self.map_file.clone()),
        }
    }

    fn get_quarchitect_forge_game_data(
        &self,
        owner: Spatial,
    ) -> Result<quarchitect::game_data::forge::GameData, String> {
        let forge_game_data = match self.get_forge_game_data(owner) {
            Some(forge_game_data) => forge_game_data,
            None => {
                return Err("No forge game data present".into());
            }
        };

        // Extract QodotGameData instance
        let forge_game_data = Variant::from_object(&forge_game_data);
        let forge_game_data = Instance::<super::ForgeGameData>::from_variant(&forge_game_data);
        let forge_game_data = match forge_game_data {
            Ok(game_data) => game_data,
            Err(err) => return Err(err.to_string()),
        };

        // Convert into quarchitect game data
        let forge_game_data: RwLockData<super::ForgeGameData> = forge_game_data.into_script();
        match forge_game_data.map(super::ForgeGameData::inner) {
            Ok(quarchitect_forge_game_data) => Ok(quarchitect_forge_game_data),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_quarchitect_game_data(
        &self,
        owner: Spatial,
    ) -> Result<quarchitect::game_data::GameData, String> {
        let qodot_game_data = match self.get_qodot_game_data(owner) {
            Some(qodot_game_data) => qodot_game_data,
            None => {
                return Err("No game data present".into());
            }
        };

        // Extract QodotGameData instance
        let qodot_game_data = Variant::from_object(&qodot_game_data);
        let qodot_game_data = Instance::<QodotGameData>::from_variant(&qodot_game_data);
        let qodot_game_data = match qodot_game_data {
            Ok(game_data) => game_data,
            Err(err) => return Err(err.to_string()),
        };

        // Convert into quarchitect game data
        let qodot_game_data: LocalCellData<QodotGameData> = qodot_game_data.into_script();
        match qodot_game_data.map(QodotGameData::to_quarchitect_game_data) {
            Ok(quarchitect_game_data) => Ok(quarchitect_game_data),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_texture_blacklist(&self) -> quarchitect::TextureBlacklist {
        quarchitect::TextureBlacklist::new(
            {
                let mut brush_textures: Vec<String> = Vec::new();
                for i in 0..self.brush_texture_blacklist.len() {
                    let brush_texture = self.brush_texture_blacklist.get(i);
                    brush_textures.push(brush_texture.to_string());
                }
                brush_textures
            },
            {
                let mut plane_textures: Vec<String> = Vec::new();
                for i in 0..self.plane_texture_blacklist.len() {
                    let plane_texture = self.plane_texture_blacklist.get(i);
                    plane_textures.push(plane_texture.to_string());
                }
                plane_textures
            },
        )
    }

    fn add_child_editor(owner: Spatial, parent: &mut Node, child: Option<Node>) {
        if let Some(mut child) = child {
            // Tag direct children with metadata identifier
            if Variant::from_object(parent) == Variant::from_object(&owner) {
                unsafe {
                    child.set_meta(CHILD_META.into(), Variant::from_bool(true));
                }
            }

            // Add child
            unsafe {
                parent.add_child(Some(child), true);
            }

            // Fetch tree
            let tree: Option<gdnative::SceneTree>;
            unsafe { tree = parent.get_tree() }

            let tree: gdnative::SceneTree = match tree {
                Some(tree) => tree,
                None => return,
            };

            // If the tree exists, fetch the edited scene root
            let edited_scene_root: Option<Node>;
            unsafe { edited_scene_root = tree.get_edited_scene_root() }

            let edited_scene_root: Node = match edited_scene_root {
                Some(edited_scene_root) => edited_scene_root,
                None => return,
            };

            // If the edited scene root exists, set it as the child's owner
            unsafe {
                child.set_owner(Some(edited_scene_root));
            }
        }
    }
}
