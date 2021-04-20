use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
    FromVariant, Instance, Map, Spatial, Variant, VariantArray,
};

use crate::{map::QuakeMap, qodot_map::MapType, texture_loader::TextureType, QodotMap};

#[methods]
impl QodotMap {
    // Change Handlers
    #[export]
    fn map_resource_changed(&mut self, mut owner: Spatial) {
        godot_print!("Map resource changed");
        if self.map_type == MapType::Resource && self.automatic_rebuild {
            unsafe {
                owner.call_deferred("set_rebuild".into(), &[Variant::from_bool(true)]);
            }
        }
    }

    // Overrides
    #[export]
    pub fn _ready(&mut self, owner: Spatial) {
        if self.map_type == MapType::Resource {
            let quake_map = Instance::<QuakeMap>::from_variant(&self.map_resource);
            match quake_map {
                Ok(quake_map) => {
                    let (base, script) = quake_map.decouple();
                    let map_revision = script.map(|script| script.get_revision(base));
                    let map_revision = match map_revision {
                        Ok(map_revision) => map_revision,
                        Err(err) => {
                            godot_error!("Error reading map revision: {:?}", err);
                            return;
                        }
                    };
                    if Some(map_revision) != self.map_revision && self.automatic_rebuild {
                        self.set_rebuild(owner, true);
                    }
                }
                Err(_err) => (),
            }
        }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: Spatial) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "QodotMap",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "rebuild",
                gdnative::GlobalConstants::TYPE_BOOL,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Game",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_GROUP),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "forge_game_data",
                gdnative::GlobalConstants::TYPE_OBJECT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                Some("Resource"),
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "qodot_game_data",
                gdnative::GlobalConstants::TYPE_OBJECT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Map",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_GROUP),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "map_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Resource,File"),
                None,
            ),
        ));

        match self.map_type {
            MapType::Resource => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "map_resource",
                        gdnative::GlobalConstants::TYPE_OBJECT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                        Some("Resource"),
                        None,
                    ),
                ));
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "automatic_rebuild",
                        gdnative::GlobalConstants::TYPE_BOOL,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            MapType::File => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "map_file",
                        gdnative::GlobalConstants::TYPE_STRING,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_GLOBAL_FILE),
                        Some("*.map"),
                        None,
                    ),
                ));
            }
        }

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Textures",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_GROUP),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "texture_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Texture Resources,Wad Resource,Wad File"),
                None,
            ),
        ));

        match self.texture_type {
            TextureType::TextureResources(_) => property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "base_search_path",
                    gdnative::GlobalConstants::TYPE_STRING,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_DIR),
                    None,
                    None,
                ),
            )),
            TextureType::WadResource(_) => property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "wad_resource",
                    gdnative::GlobalConstants::TYPE_OBJECT,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                    Some("Resource"),
                    None,
                ),
            )),
            TextureType::WadFile(_) => property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "wad_file",
                    gdnative::GlobalConstants::TYPE_STRING,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_GLOBAL_FILE),
                    Some("*.wad"),
                    None,
                ),
            )),
        };

        if let TextureType::WadResource(_) | TextureType::WadFile(_) = self.texture_type {
            property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "wad_palette_type",
                    gdnative::GlobalConstants::TYPE_INT,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                    Some("Resource,File"),
                    None,
                ),
            ));

            match self.wad_palette_type {
                crate::texture_loader::PaletteType::Resource(_) => property_list.push(
                    &Variant::from_dictionary(&crate::util::build_property_dictionary(
                        "wad_palette_resource",
                        gdnative::GlobalConstants::TYPE_OBJECT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                        Some("Resource"),
                        None,
                    )),
                ),
                crate::texture_loader::PaletteType::File(_) => property_list.push(
                    &Variant::from_dictionary(&crate::util::build_property_dictionary(
                        "wad_palette_file",
                        gdnative::GlobalConstants::TYPE_STRING,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_GLOBAL_FILE),
                        Some("*.lmp"),
                        None,
                    )),
                ),
            }
        }

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "brush_texture_blacklist",
                gdnative::GlobalConstants::TYPE_STRING_ARRAY,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "plane_texture_blacklist",
                gdnative::GlobalConstants::TYPE_STRING_ARRAY,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Materials",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_GROUP),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "default_material_data",
                gdnative::GlobalConstants::TYPE_OBJECT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                Some("Resource"),
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Build",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_GROUP),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "inverse_scale_factor",
                gdnative::GlobalConstants::TYPE_REAL,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "chunk_size",
                gdnative::GlobalConstants::TYPE_INT,
                None,
                None,
                None,
            ),
        ));

        property_list
    }
}
