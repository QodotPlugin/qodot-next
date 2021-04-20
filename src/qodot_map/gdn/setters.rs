use crate::{
    map::QuakeMap,
    qodot_map::MapType,
    texture_loader::{PaletteType, TextureType},
};
use crate::{ForgeGameData, QodotGameData, QodotMap, QuakeWad, game_data::QodotMaterialData};
use gdnative::{
    godot_error, godot_print, FromVariant, GodotString, Instance, Object, Resource,
    Spatial, StringArray, Variant, VariantArray,
};

impl QodotMap {
    // Setters
    pub fn set_rebuild(&mut self, owner: Spatial, _: bool) {
        godot_print!("Rebuild");
        self.build(owner);
    }

    pub fn set_forge_game_data(&mut self, _: Spatial, new_forge_game_data: Option<Resource>) {
        if let Some(resource) = new_forge_game_data {
            let forge_game_data_result =
                Instance::<ForgeGameData>::from_variant(&Variant::from_object(&resource));

            match forge_game_data_result {
                Ok(_forge_game_data) => self.forge_game_data = Variant::from_object(&resource),
                Err(_err) => self.forge_game_data = Variant::new(),
            }
        } else {
            self.forge_game_data = Variant::new();
        }
    }

    pub fn set_qodot_game_data(&mut self, _: Spatial, new_qodot_game_data: Option<Resource>) {
        let new_qodot_game_data = new_qodot_game_data
            .as_ref()
            .map(|qodot_game_data: &Resource| {
                Instance::<QodotGameData>::from_variant(&Variant::from_object(qodot_game_data))
            });

        if let Some(Ok(new_qodot_game_data)) = new_qodot_game_data {
            self.qodot_game_data = Variant::from_object(&new_qodot_game_data.into_base())
        } else {
            self.qodot_game_data = Variant::new()
        }
    }

    pub fn set_map_type(&mut self, mut owner: Spatial, new_map_type: i32) {
        let new_map_type: MapType = new_map_type.into();
        if self.map_type != new_map_type {
            self.map_type = new_map_type;
            self.map_revision = None;
            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    pub fn set_map_resource(&mut self, owner: Spatial, new_map_resource: Option<Resource>) {
        if let Ok(map_resource) = Instance::<QuakeMap>::from_variant(&self.map_resource) {
            unsafe {
                map_resource.into_base().disconnect(
                    "changed".into(),
                    owner.cast::<Object>(),
                    "map_resource_changed".into(),
                );
            }
        }

        if let Some(resource) = new_map_resource {
            let quake_map_result =
                Instance::<QuakeMap>::from_variant(&Variant::from_object(&resource));

            match quake_map_result {
                Ok(_quake_map) => {
                    self.map_resource = Variant::from_object(&resource);
                    if let Ok(map_resource) = Instance::<QuakeMap>::from_variant(&self.map_resource)
                    {
                        unsafe {
                            match map_resource.into_base().connect(
                                "changed".into(),
                                owner.cast::<Object>(),
                                "map_resource_changed".into(),
                                VariantArray::new(),
                                0,
                            ) {
                                Ok(()) => (),
                                Err(err) => {
                                    godot_error!(
                                        "Failed to connect map resource changed signal {:?}",
                                        err
                                    );
                                }
                            }
                        }
                    }
                }
                Err(_err) => self.map_resource = Variant::new(),
            }
        } else {
            self.map_resource = Variant::new();
        }
    }

    pub fn set_automatic_rebuild(&mut self, _owner: Spatial, new_automatic_rebuild: bool) {
        if self.automatic_rebuild != new_automatic_rebuild {
            self.automatic_rebuild = new_automatic_rebuild;
        }
    }

    pub fn set_map_revision(&mut self, _: Spatial, new_map_revision: Option<i32>) {
        self.map_revision = new_map_revision
    }

    pub fn set_map_file(&mut self, _: Spatial, new_map_file: GodotString) {
        self.map_file = new_map_file
    }

    pub fn set_texture_type(&mut self, mut owner: Spatial, new_texture_type: i32) {
        let new_texture_type: TextureType = new_texture_type.into();
        if self.texture_type != new_texture_type {
            self.texture_type = new_texture_type;

            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    pub fn set_base_search_path(&mut self, _: Spatial, new_base_search_path: GodotString) {
        self.texture_type = TextureType::TextureResources(new_base_search_path)
    }

    pub fn set_wad_resource(&mut self, _: Spatial, new_wad_resource: Option<Resource>) {
        if let Some(resource) = new_wad_resource {
            let quake_wad_result =
                Instance::<QuakeWad>::from_variant(&Variant::from_object(&resource));

            match quake_wad_result {
                Ok(_quake_wad) => {
                    self.texture_type = TextureType::WadResource(Variant::from_object(&resource))
                }
                Err(_err) => self.texture_type = TextureType::WadResource(Variant::new()),
            }
        } else {
            self.texture_type = TextureType::WadResource(Variant::new());
        }
    }

    pub fn set_wad_file(&mut self, _: Spatial, new_wad_file: GodotString) {
        self.texture_type = TextureType::WadFile(new_wad_file)
    }

    pub fn set_wad_palette_type(&mut self, mut owner: Spatial, new_wad_palette_type: i32) {
        self.wad_palette_type = new_wad_palette_type.into();

        unsafe {
            owner.property_list_changed_notify();
        }
    }

    pub fn set_wad_palette_resource(
        &mut self,
        _: Spatial,
        new_wad_palette_resource: Option<Resource>,
    ) {
        let new_wad_palette_resource = match new_wad_palette_resource {
            Some(new_wad_palette_resource) => Variant::from_object(&new_wad_palette_resource),
            None => Variant::new(),
        };

        let new_wad_palette_type = PaletteType::Resource(new_wad_palette_resource);

        if new_wad_palette_type != self.wad_palette_type {
            self.wad_palette_type = new_wad_palette_type;
        }
    }

    pub fn set_wad_palette_file(&mut self, _: Spatial, new_wad_palette_file: GodotString) {
        self.wad_palette_type = PaletteType::File(new_wad_palette_file)
    }

    pub fn set_brush_texture_blacklist(
        &mut self,
        _: Spatial,
        new_brush_texture_blacklist: StringArray,
    ) {
        self.brush_texture_blacklist = new_brush_texture_blacklist
    }

    pub fn set_plane_texture_blacklist(
        &mut self,
        _: Spatial,
        new_plane_texture_blacklist: StringArray,
    ) {
        self.plane_texture_blacklist = new_plane_texture_blacklist
    }

    pub fn set_default_material_data(
        &mut self,
        _: Spatial,
        new_default_material_data: Option<Resource>,
    ) {
        let new_default_material_data = match new_default_material_data {
            Some(new_default_material_data) => Variant::from_object(&new_default_material_data),
            None => Variant::from_object(&Instance::<QodotMaterialData>::new().into_base()),
        };

        if new_default_material_data != self.default_material_data {
            self.default_material_data = new_default_material_data;
        }
    }

    pub fn set_inverse_scale_factor(&mut self, _owner: Spatial, new_inverse_scale_factor: f32) {
        self.inverse_scale_factor = new_inverse_scale_factor;
    }

    pub fn set_chunk_size(&mut self, _owner: Spatial, new_chunk_size: i32) {
        self.chunk_size = new_chunk_size;
    }
}
