use gdnative::{
    CollisionShape, ConcavePolygonShape, ConvexPolygonShape, Node, Shape, Spatial, Vector3Array,
};
use quarchitect::Vector3;

pub fn spawn_collision_geometry(
    inverse_scale_factor: f32,
    owner: Spatial,
    parent: &Option<Node>,
    collision_geometry: &quarchitect::scene_tree::CollisionGeometry,
    origin: Vector3,
) {
    let origin = super::quake_point_to_godot_point(origin, inverse_scale_factor);

    match collision_geometry {
        quarchitect::scene_tree::CollisionGeometry::Convex(convex_collision) => {
            for convex_collision in convex_collision {
                let center = super::quake_point_to_godot_point(
                    convex_collision.center,
                    inverse_scale_factor,
                );

                let mut vertices = Vector3Array::new();
                for vertex in &convex_collision.points {
                    let vertex = super::quake_point_to_godot_point(*vertex, inverse_scale_factor);
                    vertices.push(&super::godot_vector3_from_quarchitect_vector3(
                        vertex - center,
                    ));
                }
                let mut shape = ConvexPolygonShape::new();
                shape.set_points(vertices);
                spawn_collision_shape(owner, parent, shape.cast::<Shape>(), center - origin);
            }
        }
        quarchitect::scene_tree::CollisionGeometry::Concave(concave_collision) => {
            let mut vertices = Vector3Array::new();
            for concave_collision in concave_collision {
                for index in concave_collision.indices.iter() {
                    let vertex = &concave_collision.vertices[*index];
                    let vertex = super::quake_point_to_godot_point(*vertex, inverse_scale_factor);
                    vertices.push(&super::godot_vector3_from_quarchitect_vector3(
                        vertex - origin,
                    ));
                }
            }

            let mut shape = ConcavePolygonShape::new();
            shape.set_faces(vertices);
            spawn_collision_shape(
                owner,
                parent,
                shape.cast::<Shape>(),
                Vector3::new(0.0, 0.0, 0.0),
            );
        }
        quarchitect::scene_tree::CollisionGeometry::None => (),
    }
}

fn spawn_collision_shape(
    owner: Spatial,
    parent: &Option<Node>,
    shape: Option<Shape>,
    origin: Vector3,
) -> Option<Node> {
    let mut parent: Node = match parent {
        Some(p) => *p,
        None => unsafe { owner.cast::<Node>().unwrap() },
    };

    let mut collision_shape = CollisionShape::new();
    unsafe {
        collision_shape.set_translation(super::godot_vector3_from_quarchitect_vector3(origin));
        collision_shape.set_shape(shape);
    }

    unsafe {
        let collision_shape = collision_shape.cast::<Node>();
        crate::QodotMap::add_child_editor(owner, &mut parent, collision_shape);
        collision_shape
    }
}
