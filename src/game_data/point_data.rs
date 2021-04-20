use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, GodotString,
    NativeClass, Resource, Variant, VariantArray,
};

use quarchitect::game_data::{PropertyApplicationType, PointData as QuarchitectPointData};

#[derive(Debug, Clone, NativeClass)]
#[inherit(Resource)]
#[user_data(gdnative::user_data::RwLockData<PointData>)]
#[register_with(register_point_data)]
pub struct PointData {
    data: QuarchitectPointData,
}

fn register_point_data(builder: &gdnative::init::ClassBuilder<PointData>) {
    builder
        .add_property::<i64>("spawn_type")
        .with_default(0)
        .with_getter(PointData::get_spawn_type)
        .with_setter(PointData::set_spawn_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("spawn_class_name")
        .with_default("".into())
        .with_getter(PointData::get_spawn_class_name)
        .with_setter(PointData::set_spawn_class_name)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("spawn_prefab_scene")
        .with_default("".into())
        .with_getter(PointData::get_spawn_prefab_scene)
        .with_setter(PointData::set_spawn_prefab_scene)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i64>("component_type")
        .with_default(0)
        .with_getter(PointData::get_component_type)
        .with_setter(PointData::set_component_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("component_script")
        .with_default("".into())
        .with_getter(PointData::get_component_script)
        .with_setter(PointData::set_component_script)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("property_application")
        .with_default(0)
        .with_getter(PointData::get_property_application)
        .with_setter(PointData::set_property_application)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();
}

#[gdnative::methods]
impl PointData {
    // Getters
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

    fn get_property_application(&self, _owner: Resource) -> i32 {
        match self.data.property_application_type {
            PropertyApplicationType::Properties => 0,
            PropertyApplicationType::Dictionary => 1,
            PropertyApplicationType::Metadata => 2,
        }
    }

    pub fn get_data(&self, _owner: Resource) -> &QuarchitectPointData {
        &self.data
    }

    // Setters
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

    fn set_spawn_class_name(
        script: &mut PointData,
        mut _owner: Resource,
        new_spawn_class_name: GodotString,
    ) {
        script.data.entity_type =
            quarchitect::game_data::EntityType::Class(new_spawn_class_name.to_string())
    }

    fn set_spawn_prefab_scene(
        script: &mut PointData,
        mut _owner: Resource,
        new_spawn_prefab_scene: GodotString,
    ) {
        script.data.entity_type =
            quarchitect::game_data::EntityType::Prefab(new_spawn_prefab_scene.to_string())
    }

    fn set_component_type(script: &mut PointData, mut owner: Resource, new_component_type: i64) {
        let component_type: i64 = script.data.component_type.clone().into();
        if component_type != new_component_type {
            match new_component_type {
                0 => script.data.component_type = quarchitect::game_data::ComponentType::None,
                1 => {
                    script.data.component_type =
                        quarchitect::game_data::ComponentType::Script(String::new())
                }
                _ => godot_error!("Unexpected component type"),
            }

            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    fn set_component_script(
        script: &mut PointData,
        mut _owner: Resource,
        new_component_script: GodotString,
    ) {
        script.data.component_type =
            quarchitect::game_data::ComponentType::Script(new_component_script.to_string())
    }

    fn set_property_application(
        script: &mut PointData,
        mut _owner: Resource,
        new_property_application: i32,
    ) {
        script.data.property_application_type = match new_property_application {
            0 => PropertyApplicationType::Properties,
            1 => PropertyApplicationType::Dictionary,
            2 => PropertyApplicationType::Metadata,
            _ => panic!("Unexpected property application type"),
        }
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Point Data".into())
        }

        let data = QuarchitectPointData::default();

        PointData {
            data
        }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Point Data",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
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
                "property_application",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Properties,Dictionary,Metadata"),
                None,
            ),
        ));

        property_list
    }
}
