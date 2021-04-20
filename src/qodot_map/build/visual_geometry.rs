use gdnative::{
    ArrayMesh, ColorArray, Float32Array, GodotString, Int32Array, Material, Mesh, MeshInstance,
    Node, Spatial, Variant, VariantArray, Vector2Array, Vector3Array,
};
use quarchitect::Vector3;
use std::collections::HashMap;

pub fn spawn_mesh_instance(
    owner: Spatial,
    parent: &Option<Node>,
    visual_geometry: &quarchitect::scene_tree::VisualGeometry,
) -> Option<MeshInstance> {
    let mut parent: Node = match parent {
        Some(p) => *p,
        None => unsafe { owner.cast::<Node>().unwrap() },
    };

    match visual_geometry {
        quarchitect::scene_tree::VisualGeometry::Mesh(_visual_mesh) => {
            // Create mesh and mesh instance
            let mesh = ArrayMesh::new();
            let mut mesh_instance = MeshInstance::new();
            unsafe {
                mesh_instance.set_mesh(mesh.cast::<Mesh>());
                crate::QodotMap::add_child_editor(owner, &mut parent, mesh_instance.cast::<Node>());
                Some(mesh_instance)
            }
        }
        quarchitect::scene_tree::VisualGeometry::None => None,
    }
}

pub fn populate_mesh_geometry(
    visual_geometry: &quarchitect::scene_tree::VisualGeometry,
    mesh_instance: Option<MeshInstance>,
    origin: Vector3,
    inverse_scale_factor: f32,
) {
    let origin = super::quake_point_to_godot_point(origin, inverse_scale_factor);

    match visual_geometry {
        quarchitect::scene_tree::VisualGeometry::Mesh(visual_mesh) => {
            let mesh_instance = mesh_instance.unwrap();
            let mut mesh;
            unsafe {
                mesh = mesh_instance
                    .get_mesh()
                    .unwrap()
                    .cast::<ArrayMesh>()
                    .unwrap();
            }

            for surface in visual_mesh.surfaces.iter() {
                let mut arrays = VariantArray::new();
                let blend_shapes = VariantArray::new();

                // Vertices
                arrays.push(&Variant::from_vector3_array(&surface.vertices.iter().fold(
                    Vector3Array::new(),
                    |mut acc, next| {
                        let vertex = super::quake_point_to_godot_point(*next, inverse_scale_factor);
                        let vertex = super::godot_vector3_from_quarchitect_vector3(vertex - origin);
                        acc.push(&vertex);
                        acc
                    },
                )));

                // Normals
                arrays.push(&Variant::from_vector3_array(&surface.normals.iter().fold(
                    Vector3Array::new(),
                    |mut acc, next| {
                        let normal = super::quake_direction_to_godot_direction(*next);
                        let normal = super::godot_vector3_from_quarchitect_vector3(normal);
                        acc.push(&normal);
                        acc
                    },
                )));

                // Tangents
                arrays.push(&Variant::from_float32_array(&surface.tangents.iter().fold(
                    Float32Array::new(),
                    |mut acc, next| {
                        let (tangent, flip_binormal) = next;
                        let tangent = super::quake_direction_to_godot_direction(*tangent);
                        acc.push(tangent.x());
                        acc.push(tangent.y());
                        acc.push(tangent.z());
                        acc.push(*flip_binormal);
                        acc
                    },
                )));

                // Colors
                match &surface.colors {
                    Some(surface_colors) => {
                        arrays.push(&Variant::from_color_array(&surface_colors.iter().fold(
                            ColorArray::new(),
                            |mut acc, next| {
                                let color = gdnative::Color::rgb(next.r, next.g, next.b);
                                acc.push(&color);
                                acc
                            },
                        )));
                    }
                    None => arrays.push(&Variant::new()),
                };

                // UVs
                match &surface.uvs {
                    Some(surface_uvs) => {
                        arrays.push(&Variant::from_vector2_array(&surface_uvs.iter().fold(
                            Vector2Array::new(),
                            |mut acc, next| {
                                let uv = gdnative::Vector2::new(next.x(), next.y());
                                acc.push(&uv);
                                acc
                            },
                        )));
                    }
                    None => arrays.push(&Variant::new()),
                }

                arrays.push(&Variant::new()); // UV2s
                arrays.push(&Variant::new()); // Bones
                arrays.push(&Variant::new()); // Weights

                // Indices
                arrays.push(&Variant::from_int32_array(&surface.indices.iter().fold(
                    Int32Array::new(),
                    |mut acc, next| {
                        acc.push(*next as i32);
                        acc
                    },
                )));

                // Add Surface
                mesh.add_surface_from_arrays(
                    Mesh::PRIMITIVE_TRIANGLES,
                    arrays,
                    blend_shapes,
                    31744,
                );
            }
        }
        quarchitect::scene_tree::VisualGeometry::None => (),
    }
}

const TEXTURE_SLOTS: &[i64] = &[
    gdnative::SpatialMaterial::TEXTURE_ALBEDO,
    gdnative::SpatialMaterial::TEXTURE_METALLIC,
    gdnative::SpatialMaterial::TEXTURE_ROUGHNESS,
    gdnative::SpatialMaterial::TEXTURE_EMISSION,
    gdnative::SpatialMaterial::TEXTURE_NORMAL,
    gdnative::SpatialMaterial::TEXTURE_RIM,
    gdnative::SpatialMaterial::TEXTURE_CLEARCOAT,
    gdnative::SpatialMaterial::TEXTURE_FLOWMAP,
    gdnative::SpatialMaterial::TEXTURE_AMBIENT_OCCLUSION,
    gdnative::SpatialMaterial::TEXTURE_DEPTH,
    gdnative::SpatialMaterial::TEXTURE_SUBSURFACE_SCATTERING,
    gdnative::SpatialMaterial::TEXTURE_TRANSMISSION,
    gdnative::SpatialMaterial::TEXTURE_REFRACTION,
    gdnative::SpatialMaterial::TEXTURE_DETAIL_MASK,
    gdnative::SpatialMaterial::TEXTURE_DETAIL_ALBEDO,
    gdnative::SpatialMaterial::TEXTURE_DETAIL_NORMAL,
];

const TEXTURE_SETTINGS: &[&str] = &[
    "qodot/textures/albedo_pattern",
    "qodot/textures/metallic_pattern",
    "qodot/textures/roughness_pattern",
    "qodot/textures/emission_pattern",
    "qodot/textures/normal_pattern",
    "qodot/textures/rim_pattern",
    "qodot/textures/clearcoat_pattern",
    "qodot/textures/flowmap_pattern",
    "qodot/textures/ambient_occlusion_pattern",
    "qodot/textures/depth_pattern",
    "qodot/textures/subsurface_scattering_pattern",
    "qodot/textures/transmission_pattern",
    "qodot/textures/refraction_pattern",
    "qodot/textures/detail_mask_pattern",
    "qodot/textures/detail_albedo_pattern",
    "qodot/textures/detail_normal_pattern",
];

pub fn populate_mesh_materials(
    textures: &HashMap<String, (Variant, HashMap<String, Variant>)>,
    default_material: Option<Material>,
    default_spatial_material_texture_param: i32,
    default_shader_material_texture_param: &GodotString,
    visual_geometry: &quarchitect::scene_tree::VisualGeometry,
    mesh_instance: Option<MeshInstance>,
) {
    match visual_geometry {
        quarchitect::scene_tree::VisualGeometry::Mesh(visual_mesh) => {
            let mesh_instance = mesh_instance.unwrap();
            let mut mesh;
            unsafe {
                mesh = mesh_instance
                    .get_mesh()
                    .unwrap()
                    .cast::<ArrayMesh>()
                    .unwrap();
            }

            let project_settings = gdnative::ProjectSettings::godot_singleton();

            let iter_slots = TEXTURE_SLOTS.iter().cloned();
            let iter_patterns = TEXTURE_SETTINGS.iter().map(|setting|{
                (&project_settings).get_setting(setting.into()).to_string()
            });
            let texture_pairs = iter_slots.zip(iter_patterns);
            let texture_pairs: HashMap<i64, String> = texture_pairs.collect();

            for (i, surface) in visual_mesh.surfaces.iter().enumerate() {
                if let Some(texture) = &surface.texture {
                    let index = i as i64;

                    mesh.surface_set_name(index, texture.into());

                    if let Some(default_material) = &default_material {
                        if let Some(material) = default_material.duplicate(false) {
                            if let Some(mut spatial_material) =
                                material.cast::<gdnative::SpatialMaterial>()
                            {
                                if let Some((texture_var, extra)) = textures.get(texture) {
                                    if default_spatial_material_texture_param > 0 {
                                        spatial_material.set_texture(
                                            (default_spatial_material_texture_param - 1) as i64,
                                            texture_var.try_to_object::<gdnative::Texture>(),
                                        );
                                    }
                                    
                                    for (key, value) in extra {
                                        let texture_name = texture.split('/').last().unwrap();

                                        // Match against key
                                        for (spatial_material_texture, pattern) in &texture_pairs {
                                            let candidate = pattern.replace("$TEXTURE", &texture_name);
                                            if candidate.cmp(key) == std::cmp::Ordering::Equal {
                                                spatial_material.set_texture(
                                                    *spatial_material_texture,
                                                    value.try_to_object::<gdnative::Texture>(),
                                                )
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(mut shader_material) =
                                material.cast::<gdnative::ShaderMaterial>()
                            {
                                if let Some((texture, extra)) = textures.get(texture) {
                                    shader_material.set_shader_param(
                                        default_shader_material_texture_param.clone(),
                                        texture.clone(),
                                    );

                                    for (key, value) in extra {
                                        shader_material.set_shader_param(key.into(), value.clone());
                                    }
                                }
                            }

                            mesh.surface_set_material(index, material.cast::<Material>());
                        }
                    }
                }
            }
        }
        quarchitect::scene_tree::VisualGeometry::None => (),
    }
}
