#![allow(clippy::not_unsafe_ptr_arg_deref)] // Silence gdnative clippy warnings

use gdnative::{
    godot_error, godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init,
};

pub mod util;

mod game_data;
mod map;
mod qodot_map;
mod texture_loader;
mod wad;

use game_data::{
    forge::{ForgeChoice, ForgeEntity, ForgeGameData, ForgeMetadata, ForgeProperty},
    BrushData, PointData, QodotEntity, QodotGameData, QodotMaterialData, QodotWorldspawnLayer,
};
use map::QuakeMap;
use qodot_map::QodotBuildWorker;
use qodot_map::QodotMap;
use wad::QuakePalette;
use wad::QuakeWad;
use wad::QuakeWadDebug;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_tool_class::<QuakeMap>();
    handle.add_tool_class::<QuakePalette>();
    handle.add_tool_class::<QuakeWad>();
    handle.add_tool_class::<QuakeWadDebug>();
    handle.add_tool_class::<PointData>();
    handle.add_tool_class::<BrushData>();

    handle.add_tool_class::<QodotGameData>();
    handle.add_tool_class::<QodotMaterialData>();
    handle.add_tool_class::<QodotEntity>();
    handle.add_tool_class::<QodotWorldspawnLayer>();

    handle.add_tool_class::<ForgeGameData>();
    handle.add_tool_class::<ForgeEntity>();
    handle.add_tool_class::<ForgeMetadata>();
    handle.add_tool_class::<ForgeProperty>();
    handle.add_tool_class::<ForgeChoice>();

    handle.add_tool_class::<QodotMap>();
    handle.add_tool_class::<QodotBuildWorker>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
