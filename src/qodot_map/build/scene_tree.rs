use gdnative::{godot_error, Dictionary, GodotString, Node, ResourceLoader, Spatial, Variant};
use quarchitect::game_data::{Properties, Property};

pub fn spawn_scene_tree_actor(
    inverse_scale_factor: f32,
    owner: Spatial,
    parent: &Option<Node>,
    scene_tree: &quarchitect::scene_tree::SceneTreeNode,
    actor: &quarchitect::scene_tree::Actor,
) -> Option<Node> {
    let mut parent: Node = match parent {
        Some(p) => *p,
        None => unsafe { owner.cast::<Node>().unwrap() },
    };

    let entity = match &actor.entity_type {
        quarchitect::game_data::EntityType::Placeholder => super::entities::spawn_class_entity(
            owner,
            &mut parent,
            "Position3D",
            scene_tree.origin,
            inverse_scale_factor,
        ),
        quarchitect::game_data::EntityType::Class(class_name) => {
            super::entities::spawn_class_entity(
                owner,
                &mut parent,
                class_name,
                scene_tree.origin,
                inverse_scale_factor,
            )
        }
        quarchitect::game_data::EntityType::Prefab(prefab_name) => {
            super::entities::spawn_prefab_entity(
                owner,
                &mut parent,
                prefab_name,
                scene_tree.origin,
                inverse_scale_factor,
            )
        }
    };

    if let Some(mut object) = entity {
        let Properties(properties) = &actor.properties;

        let object_name: GodotString;
        if let Some(Property::String(tb_name)) = properties.get("_tb_name") {
            object_name = GodotString::from_str(tb_name);
        }
        else {
            object_name = GodotString::from_str(&actor.name).capitalize();
        }

        unsafe { object.set_name(object_name) }

        if let Some(component_class) = &actor.component_class {
            let component_script = ResourceLoader::godot_singleton().load(
                component_class.into(),
                "Script".into(),
                false,
            );

            if let Some(component_script) = component_script {
                unsafe { object.set_script(Some(component_script.to_reference())) }

                match actor.property_application_type {
                    quarchitect::game_data::PropertyApplicationType::Properties => {
                        populate_properties(actor, object)
                    }
                    quarchitect::game_data::PropertyApplicationType::Dictionary => {
                        populate_property_dictionary(actor, object)
                    }
                    quarchitect::game_data::PropertyApplicationType::Metadata => {
                        populate_property_metadata(actor, object)
                    }
                }
            };
        }
    }

    if entity.is_none() {
        godot_error!("Failed to spawn object of class {:?}", &actor.entity_type);
    }

    entity
}

fn populate_properties(actor: &quarchitect::scene_tree::Actor, mut object: Node) {
    let Properties(properties) = &actor.properties;
    for (key, value) in properties {
        unsafe {
            object.set(key.into(), quarchitect_property_to_variant(value));
        }
    }
}

fn populate_property_dictionary(actor: &quarchitect::scene_tree::Actor, mut object: Node) {
    let mut property_dict = Dictionary::new();
    let Properties(properties) = &actor.properties;
    for (key, value) in properties {
        let value = quarchitect_property_to_variant(value);
        property_dict.set(&Variant::from_str(&key), &value)
    }

    unsafe {
        object.set(
            "properties".into(),
            Variant::from_dictionary(&property_dict),
        );
    }
}

fn populate_property_metadata(actor: &quarchitect::scene_tree::Actor, mut object: Node) {
    let Properties(properties) = &actor.properties;
    for (key, value) in properties {
        unsafe {
            object.set_meta(key.into(), quarchitect_property_to_variant(value));
        }
    }
}

fn quarchitect_property_to_variant(property: &quarchitect::game_data::Property) -> Variant {
    match property {
        quarchitect::game_data::Property::Integer(value) => Variant::from_i64(*value as i64),
        quarchitect::game_data::Property::Float(value) => Variant::from_f64(*value as f64),
        quarchitect::game_data::Property::Vector3(value) => {
            Variant::from_vector3(&gdnative::Vector3::new(value.x(), value.y(), value.z()))
        }
        quarchitect::game_data::Property::String(value) => Variant::from_str(&value),
        quarchitect::game_data::Property::Color(value) => {
            Variant::from_color(&gdnative::Color::rgb(value.r, value.g, value.b))
        }
        quarchitect::game_data::Property::Choices(value) => Variant::from_i64(*value as i64),
        quarchitect::game_data::Property::Flags(value) => Variant::from_i64(*value as i64),
        quarchitect::game_data::Property::TargetSource => Variant::new(),
        quarchitect::game_data::Property::TargetDestination => Variant::new(),
    }
}
