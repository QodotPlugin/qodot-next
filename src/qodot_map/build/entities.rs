use gdnative::{ClassDB, Node, Spatial, Variant};
use quarchitect::Vector3;

pub fn spawn_class_entity(
    owner: Spatial,
    parent: &mut Node,
    class_name: &str,
    origin: Vector3,
    inverse_scale_factor: f32
) -> Option<Node> {
    let class_db = ClassDB::godot_singleton();
    let instance: Variant = class_db.instance(class_name.into());

    if let Some(mut entity) = instance.try_to_object::<Spatial>() {
        unsafe {
            let origin = super::quake_point_to_godot_point(origin, inverse_scale_factor);
            entity.set_translation(super::godot_vector3_from_quarchitect_vector3(origin));
        }
    }

    let instance = instance.try_to_object::<Node>();
    crate::QodotMap::add_child_editor(owner, parent, instance);
    instance
}

pub fn spawn_prefab_entity(
    _owner: Spatial,
    _parent: &mut Node,
    prefab_name: &str,
    _origin: Vector3,
    _inverse_scale_factor: f32
) -> Option<Node> {
    println!("TODO: Implement Prefab Entities. Tried to spawn {:#?}", prefab_name);
    None
}
