pub mod collision_geometry;
pub mod entities;
pub mod scene_tree;
pub mod visual_geometry;
pub mod worker;

pub fn quake_point_to_godot_point(
    point: quarchitect::Vector3,
    inverse_scale_factor: f32,
) -> quarchitect::Vector3 {
    quake_direction_to_godot_direction(point) / inverse_scale_factor
}

pub fn quake_direction_to_godot_direction(dir: quarchitect::Vector3) -> quarchitect::Vector3 {
    let rot = quarchitect::Quat::from_axis_angle(
        quarchitect::Vector3::new(-1.0, 0.0, 0.0),
        90.0_f32.to_radians(),
    );
    rot * dir
}

pub fn godot_vector3_from_quarchitect_vector3(vec: quarchitect::Vector3) -> gdnative::Vector3 {
    let (x, y, z) = vec.into();
    gdnative::Vector3::new(x, y, z)
}
