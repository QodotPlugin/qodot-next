use gdnative::{GodotString, Resource, Spatial, StringArray};

use crate::QodotMap;

impl QodotMap {
    pub fn get_rebuild(&self, _: Spatial) -> bool {
        false
    }

    pub fn get_forge_game_data(&self, _: Spatial) -> Option<Resource> {
        self.forge_game_data.try_to_object()
    }

    pub fn get_qodot_game_data(&self, _: Spatial) -> Option<Resource> {
        self.qodot_game_data.try_to_object()
    }

    pub fn get_map_type(&self, _: Spatial) -> i32 {
        self.map_type.into()
    }

    pub fn get_map_resource(&self, _: Spatial) -> Option<Resource> {
        self.map_resource.try_to_object()
    }

    pub fn get_automatic_rebuild(&self, _: Spatial) -> bool {
        self.automatic_rebuild
    }

    pub fn get_map_revision(&self, _: Spatial) -> Option<i32> {
        self.map_revision
    }

    pub fn get_map_file(&self, _: Spatial) -> GodotString {
        self.map_file.clone()
    }

    pub fn get_texture_type(&self, _: Spatial) -> i32 {
        self.texture_type.clone().into()
    }

    pub fn get_base_search_path(&self, _: Spatial) -> GodotString {
        match &self.texture_type {
            crate::texture_loader::TextureType::TextureResources(base_path) => base_path.clone(),
            _ => "res://".into(),
        }
    }

    pub fn get_wad_resource(&self, _: Spatial) -> Option<Resource> {
        match &self.texture_type {
            crate::texture_loader::TextureType::WadResource(wad_resource) => {
                wad_resource.try_to_object::<Resource>()
            }
            _ => None,
        }
    }

    pub fn get_wad_file(&self, _: Spatial) -> GodotString {
        match &self.texture_type {
            crate::texture_loader::TextureType::WadFile(wad_file) => wad_file.clone(),
            _ => "res://".into(),
        }
    }

    pub fn get_wad_palette_type(&self, _: Spatial) -> i32 {
        match self.wad_palette_type {
            crate::texture_loader::PaletteType::Resource(_) => 0,
            crate::texture_loader::PaletteType::File(_) => 1,
        }
    }

    pub fn get_wad_palette_resource(&self, _: Spatial) -> Option<Resource> {
        match &self.wad_palette_type {
            crate::texture_loader::PaletteType::Resource(wad_palette_resource) => {
                wad_palette_resource.try_to_object::<Resource>()
            }
            _ => None,
        }
    }

    pub fn get_wad_palette_file(&self, _: Spatial) -> GodotString {
        match &self.wad_palette_type {
            crate::texture_loader::PaletteType::File(wad_palette_file) => wad_palette_file.clone(),
            _ => GodotString::new(),
        }
    }

    pub fn get_brush_texture_blacklist(&self, _: Spatial) -> &StringArray {
        &self.brush_texture_blacklist
    }

    pub fn get_plane_texture_blacklist(&self, _: Spatial) -> &StringArray {
        &self.plane_texture_blacklist
    }

    pub fn get_default_material_data(&self, _: Spatial) -> Option<Resource> {
        self.default_material_data.try_to_object::<Resource>()
    }
    pub fn get_inverse_scale_factor(&self, _: Spatial) -> f32 {
        self.inverse_scale_factor
    }

    pub fn get_chunk_size(&self, _: Spatial) -> i32 {
        self.chunk_size
    }
}
