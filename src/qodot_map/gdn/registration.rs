use crate::QodotMap;
use gdnative::{
    init::ClassBuilder, GodotString, Resource, StringArray,
};

pub fn register_qodot_map(builder: &ClassBuilder<QodotMap>) {
    builder
        .add_property::<GodotString>("Qodot Map")
        .with_default(GodotString::default())
        .with_usage(gdnative::init::PropertyUsage::CATEGORY)
        .done();
    
    builder
        .add_property::<bool>("rebuild")
        .with_default(false)
        .with_getter(QodotMap::get_rebuild)
        .with_setter(QodotMap::set_rebuild)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("forge_game_data")
        .with_default(None)
        .with_getter(QodotMap::get_forge_game_data)
        .with_setter(QodotMap::set_forge_game_data)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("qodot_game_data")
        .with_default(None)
        .with_getter(QodotMap::get_qodot_game_data)
        .with_setter(QodotMap::set_qodot_game_data)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("map_type")
        .with_default(0)
        .with_getter(QodotMap::get_map_type)
        .with_setter(QodotMap::set_map_type)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("map_resource")
        .with_default(None)
        .with_getter(QodotMap::get_map_resource)
        .with_setter(QodotMap::set_map_resource)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<bool>("automatic_rebuild")
        .with_default(true)
        .with_getter(QodotMap::get_automatic_rebuild)
        .with_setter(QodotMap::set_automatic_rebuild)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<i32>>("map_revision")
        .with_default(None)
        .with_getter(QodotMap::get_map_revision)
        .with_setter(QodotMap::set_map_revision)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("map_file")
        .with_default("".into())
        .with_getter(QodotMap::get_map_file)
        .with_setter(QodotMap::set_map_file)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("texture_type")
        .with_default(0)
        .with_getter(QodotMap::get_texture_type)
        .with_setter(QodotMap::set_texture_type)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("base_search_path")
        .with_default("res://".into())
        .with_getter(QodotMap::get_base_search_path)
        .with_setter(QodotMap::set_base_search_path)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("wad_resource")
        .with_default(None)
        .with_getter(QodotMap::get_wad_resource)
        .with_setter(QodotMap::set_wad_resource)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("wad_file")
        .with_default("res://".into())
        .with_getter(QodotMap::get_wad_file)
        .with_setter(QodotMap::set_wad_file)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("wad_palette_type")
        .with_default(0)
        .with_getter(QodotMap::get_wad_palette_type)
        .with_setter(QodotMap::set_wad_palette_type)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("wad_palette_resource")
        .with_default(None)
        .with_getter(QodotMap::get_wad_palette_resource)
        .with_setter(QodotMap::set_wad_palette_resource)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("wad_palette_file")
        .with_default("res://".into())
        .with_getter(QodotMap::get_wad_palette_file)
        .with_setter(QodotMap::set_wad_palette_file)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<StringArray>("brush_texture_blacklist")
        .with_default({
            let mut brush_texture_blacklist = StringArray::new();
            brush_texture_blacklist.push(&GodotString::from_str(&"special/clip"));
            brush_texture_blacklist
        })
        .with_ref_getter(QodotMap::get_brush_texture_blacklist)
        .with_setter(QodotMap::set_brush_texture_blacklist)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<StringArray>("plane_texture_blacklist")
        .with_default({
            let mut plane_texture_blacklist = StringArray::new();
            plane_texture_blacklist.push(&GodotString::from_str(&"special/skip"));
            plane_texture_blacklist.push(&GodotString::from_str(&"__TB_empty"));
            plane_texture_blacklist
        })
        .with_ref_getter(QodotMap::get_plane_texture_blacklist)
        .with_setter(QodotMap::set_plane_texture_blacklist)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("default_material_data")
        .with_default(None)
        .with_getter(QodotMap::get_default_material_data)
        .with_setter(QodotMap::set_default_material_data)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<f32>("inverse_scale_factor")
        .with_default(16.0)
        .with_getter(QodotMap::get_inverse_scale_factor)
        .with_setter(QodotMap::set_inverse_scale_factor)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("chunk_size")
        .with_default(64)
        .with_getter(QodotMap::get_chunk_size)
        .with_setter(QodotMap::set_chunk_size)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}
