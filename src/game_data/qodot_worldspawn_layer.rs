use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods, FromVariant,
    GodotString, Instance, Map, NativeClass, Resource, Variant, VariantArray,
};

use quarchitect::game_data::WorldspawnLayer as QuarchitectWorldspawnLayer;

// TODO-3: Research using traits to minimize code duplication with point and brush data

#[derive(Debug, NativeClass)]
#[inherit(Resource)]
#[user_data(gdnative::user_data::RwLockData<QodotWorldspawnLayer>)]
#[register_with(register_qodot_worldspawn_layer)]
pub struct QodotWorldspawnLayer {
    data: QuarchitectWorldspawnLayer,
}

fn register_qodot_worldspawn_layer(builder: &gdnative::init::ClassBuilder<QodotWorldspawnLayer>) {
    builder
        .add_property::<GodotString>("texture")
        .with_default(GodotString::default())
        .with_getter(QodotWorldspawnLayer::get_texture)
        .with_setter(QodotWorldspawnLayer::set_texture)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i64>("spawn_type")
        .with_default(0)
        .with_getter(QodotWorldspawnLayer::get_spawn_type)
        .with_setter(QodotWorldspawnLayer::set_spawn_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("spawn_class_name")
        .with_default("".into())
        .with_getter(QodotWorldspawnLayer::get_spawn_class_name)
        .with_setter(QodotWorldspawnLayer::set_spawn_class_name)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("spawn_prefab_scene")
        .with_default("".into())
        .with_getter(QodotWorldspawnLayer::get_spawn_prefab_scene)
        .with_setter(QodotWorldspawnLayer::set_spawn_prefab_scene)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i64>("component_type")
        .with_default(0)
        .with_getter(QodotWorldspawnLayer::get_component_type)
        .with_setter(QodotWorldspawnLayer::set_component_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("component_script")
        .with_default("".into())
        .with_getter(QodotWorldspawnLayer::get_component_script)
        .with_setter(QodotWorldspawnLayer::set_component_script)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i64>("visual_type")
        .with_default(1)
        .with_getter(QodotWorldspawnLayer::get_visual_type)
        .with_setter(QodotWorldspawnLayer::set_visual_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i64>("collision_type")
        .with_default(2)
        .with_getter(QodotWorldspawnLayer::get_collision_type)
        .with_setter(QodotWorldspawnLayer::set_collision_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();
}

#[methods]
impl QodotWorldspawnLayer {
    // Getters
    fn get_texture(&self, _: Resource) -> GodotString {
        self.data.texture.clone().into()
    }

    fn get_spawn_type(&self, _owner: Resource) -> i64 {
        match self.data.entity_type {
            quarchitect::game_data::EntityType::Class(_) => 0,
            quarchitect::game_data::EntityType::Prefab(_) => 1,
            _ => panic!("Unexpected spawn type"),
        }
    }

    fn get_spawn_class_name(&self, _owner: Resource) -> GodotString {
        match &self.data.entity_type {
            quarchitect::game_data::EntityType::Class(class_name) => class_name.into(),
            _ => GodotString::new(),
        }
    }

    fn get_spawn_prefab_scene(&self, _owner: Resource) -> GodotString {
        match &self.data.entity_type {
            quarchitect::game_data::EntityType::Prefab(prefab_scene) => prefab_scene.into(),
            _ => GodotString::new(),
        }
    }

    fn get_component_type(&self, _owner: Resource) -> i64 {
        match &self.data.component_type {
            quarchitect::game_data::ComponentType::None => 0,
            quarchitect::game_data::ComponentType::Script(_) => 1,
        }
    }

    fn get_component_script(&self, _owner: Resource) -> GodotString {
        match &self.data.component_type {
            quarchitect::game_data::ComponentType::Script(component_script) => {
                component_script.into()
            }
            _ => GodotString::new(),
        }
    }

    pub fn get_visual_type(&self, _: Resource) -> i64 {
        self.data.visual_type.into()
    }

    pub fn get_collision_type(&self, _: Resource) -> i64 {
        self.data.collision_type.into()
    }

    // Setters
    fn set_texture(&mut self, _: Resource, new_texture: GodotString) {
        self.data.texture = new_texture.to_string()
    }

    fn set_spawn_type(&mut self, mut owner: Resource, new_spawn_type: i64) {
        let entity_type: i64 = self.data.entity_type.clone().into();
        let entity_type = entity_type - 1;
        if entity_type != new_spawn_type {
            match new_spawn_type {
                0 => {
                    self.data.entity_type = quarchitect::game_data::EntityType::Class(String::new())
                }
                1 => {
                    self.data.entity_type =
                        quarchitect::game_data::EntityType::Prefab(String::new())
                }
                _ => godot_error!("Unexpected entity type"),
            }

            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    fn set_spawn_class_name(&mut self, mut _owner: Resource, new_spawn_class_name: GodotString) {
        self.data.entity_type =
            quarchitect::game_data::EntityType::Class(new_spawn_class_name.to_string())
    }

    fn set_spawn_prefab_scene(
        &mut self,
        mut _owner: Resource,
        new_spawn_prefab_scene: GodotString,
    ) {
        self.data.entity_type =
            quarchitect::game_data::EntityType::Prefab(new_spawn_prefab_scene.to_string())
    }

    fn set_component_type(&mut self, mut owner: Resource, new_component_type: i64) {
        let component_type: i64 = self.data.component_type.clone().into();
        if component_type != new_component_type {
            match new_component_type {
                0 => self.data.component_type = quarchitect::game_data::ComponentType::None,
                1 => {
                    self.data.component_type =
                        quarchitect::game_data::ComponentType::Script(String::new())
                }
                _ => godot_error!("Unexpected component type"),
            }

            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    fn set_component_script(&mut self, mut _owner: Resource, new_component_script: GodotString) {
        self.data.component_type =
            quarchitect::game_data::ComponentType::Script(new_component_script.to_string())
    }

    pub fn set_visual_type(&mut self, mut _owner: Resource, new_visual_type: i64) {
        self.data.visual_type = new_visual_type.into();
    }

    pub fn set_collision_type(&mut self, mut _owner: Resource, new_collision_type: i64) {
        self.data.collision_type = new_collision_type.into();
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Worldspawn Layer".into())
        }

        let data = QuarchitectWorldspawnLayer::default();

        QodotWorldspawnLayer { data }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Qodot Worldspawn Layer",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "texture",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "spawn_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Class,Prefab"),
                None,
            ),
        ));

        match self.data.entity_type {
            quarchitect::game_data::EntityType::Class(_) => property_list.push(
                &Variant::from_dictionary(&crate::util::build_property_dictionary(
                    "spawn_class_name",
                    gdnative::GlobalConstants::TYPE_STRING,
                    None,
                    None,
                    None,
                )),
            ),
            quarchitect::game_data::EntityType::Prefab(_) => property_list.push(
                &Variant::from_dictionary(&crate::util::build_property_dictionary(
                    "spawn_prefab_scene",
                    gdnative::GlobalConstants::TYPE_STRING,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_FILE),
                    Some("*.tscn,*.scn,*.res"),
                    None,
                )),
            ),
            _ => (),
        }

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "component_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("None,Script"),
                None,
            ),
        ));

        if let quarchitect::game_data::ComponentType::Script(_) = self.data.component_type {
            property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "component_script",
                    gdnative::GlobalConstants::TYPE_STRING,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_FILE),
                    Some("*.gd,*.gdns,*.vs"),
                    None,
                ),
            ));
        }

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "visual_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("None,Mesh"),
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "collision_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("None,Convex,Concave"),
                None,
            ),
        ));

        property_list
    }

    pub fn qodot_worldspawn_layer_to_quarchitect_worldspawn_layer(
        worldspawn_layer: &Variant,
    ) -> Option<quarchitect::game_data::WorldspawnLayer> {
        // Entity
        let worldspawn_layer: Result<Instance<QodotWorldspawnLayer>, gdnative::FromVariantError> =
            Instance::<QodotWorldspawnLayer>::from_variant(&worldspawn_layer);

        let worldspawn_layer = match worldspawn_layer {
            Ok(worldspawn_layer) => worldspawn_layer,
            Err(err) => {
                godot_error!("Failed to load worldspawn layer data: {:?}", err);
                return None;
            }
        };

        let (base, script): (
            Resource,
            gdnative::user_data::RwLockData<QodotWorldspawnLayer>,
        ) = worldspawn_layer.decouple();

        script
            .map(
                |worldspawn_layer: &QodotWorldspawnLayer| QuarchitectWorldspawnLayer {
                    name: base.get_name().to_string(),
                    ..worldspawn_layer.data.clone()
                },
            )
            .ok()
    }
}
