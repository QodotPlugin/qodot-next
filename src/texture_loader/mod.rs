use crate::wad::{QuakePalette, QuakeWad};
use gdnative::{
    godot_error, godot_print, Directory, FromVariant, GodotError, GodotString, Instance,
    ProjectSettings, Variant,
};
use quarchitect::wad::palette::Palette;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PaletteType {
    Resource(Variant),
    File(GodotString),
}

impl Into<i32> for PaletteType {
    fn into(self) -> i32 {
        match self {
            PaletteType::Resource(_) => 0,
            PaletteType::File(_) => 1,
        }
    }
}

impl From<i32> for PaletteType {
    fn from(val: i32) -> PaletteType {
        match val {
            0 => PaletteType::Resource(Variant::new()),
            1 => PaletteType::File(GodotString::new()),
            _ => panic!("Invalid palette type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextureType {
    TextureResources(GodotString),
    WadResource(Variant),
    WadFile(GodotString),
}

impl Into<i32> for TextureType {
    fn into(self) -> i32 {
        match self {
            TextureType::TextureResources(_) => 0,
            TextureType::WadResource(_) => 1,
            TextureType::WadFile(_) => 2,
        }
    }
}

impl From<i32> for TextureType {
    fn from(val: i32) -> TextureType {
        match val {
            0 => TextureType::TextureResources("res://".into()),
            1 => TextureType::WadResource(Variant::new()),
            2 => TextureType::WadFile("res://".into()),
            _ => panic!("Invalid texture type"),
        }
    }
}

pub struct TextureInfo {
    pub quarchitect_data: quarchitect::Texture,
    pub gdnative_data: gdnative::Texture,
    pub gdnative_extra: HashMap<String, gdnative::Texture>,
}

pub fn load_textures(
    texture_type: &TextureType,
    palette_type: &PaletteType,
) -> Result<HashMap<String, TextureInfo>, String> {
    let palette = match palette_type {
        PaletteType::Resource(palette_resource) => load_palette_resource(palette_resource),
        PaletteType::File(palette_file) => load_palette_file(palette_file),
    };

    let palette = palette.ok();

    match texture_type {
        TextureType::TextureResources(base_path) => load_texture_resources(base_path),
        TextureType::WadResource(wad_resource) => load_wad_resource(wad_resource, palette),
        TextureType::WadFile(wad_file) => load_wad_file(wad_file, palette),
    }
}

pub fn load_texture_resources(
    base_path: &GodotString,
) -> Result<HashMap<String, TextureInfo>, String> {
    godot_print!("TODO-1: Refactor into de-nested functions");
    godot_print!("TODO-1: Load material data resources using the same search path pattern as textures");

    let mut dir = gdnative::Directory::new();
    let mut texture_info = HashMap::new();

    let directories = match list_directories(&mut dir, base_path.clone()) {
        Ok(directories) => directories,
        Err(err) => return Err(format!("Error listing directories: {:?}", err)),
    };

    for directory in directories {
        let texture_path = base_path.to_string() + &directory.to_string() + "/";
        let texture_path = GodotString::from(texture_path);
        let groups = match list_directories(&mut dir, texture_path.clone()) {
            Ok(groups) => groups,
            Err(err) => return Err(format!("Error listing directories: {:?}", err)),
        };

        for group in groups {
            let group_path = texture_path.to_string() + &group.to_string() + "/";
            let group_path = GodotString::from(group_path);
            let files = match list_files(&mut dir, group_path.clone()) {
                Ok(files) => files,
                Err(err) => return Err(format!("Error listing files: {:?}", err)),
            };
            
            for file in files {
                let file_path = group_path.to_string() + &file.to_string();
                let file_path = GodotString::from(file_path);
                if gdnative::ResourceLoader::godot_singleton()
                    .exists(file_path.clone(), "Texture".into())
                {
                    let resource = gdnative::ResourceLoader::godot_singleton().load(
                        file_path.clone(),
                        "Texture".into(),
                        false,
                    );

                    if let Some(resource) = resource {
                        if let Some(texture) = resource.cast::<gdnative::Texture>() {
                            let texture_name =
                                file.to_string().split('.').next().unwrap().to_string();

                            // Read extra textures into new entries naming pattern group/texture/extra
                            let mut extra: HashMap<String, gdnative::Texture> = HashMap::new();

                            let extra_dir = group_path.to_string() + &texture_name;
                            let extra_dir = GodotString::from_str(&extra_dir);
                            if dir.dir_exists(extra_dir.clone()) {
                                let files = match list_files(&mut dir, extra_dir.clone()) {
                                    Ok(files) => files,
                                    Err(err) => {
                                        return Err(format!("Error listing files: {:?}", err))
                                    }
                                };

                                for file in files {
                                    let file_path = extra_dir.to_string() + "/" + &file.to_string();
                                    let file_path = GodotString::from(file_path);
                                    if gdnative::ResourceLoader::godot_singleton()
                                        .exists(file_path.clone(), "Texture".into())
                                    {
                                        let resource = gdnative::ResourceLoader::godot_singleton()
                                            .load(file_path.clone(), "Texture".into(), false);

                                        if let Some(resource) = resource {
                                            if let Some(texture) =
                                                resource.cast::<gdnative::Texture>()
                                            {
                                                let file = file.to_string();
                                                let extra_name = file.split('.').next().unwrap();

                                                extra.insert(
                                                    extra_name.to_string(),
                                                    texture,
                                                );
                                            }
                                        }
                                    }
                                }
                            }

                            texture_info.insert(
                                group.to_string() + "/" + &texture_name,
                                TextureInfo {
                                    quarchitect_data: quarchitect::Texture::new(
                                        texture.get_width() as u32,
                                        texture.get_height() as u32,
                                    ),
                                    gdnative_data: texture,
                                    gdnative_extra: extra
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(texture_info)
}

pub fn load_palette_resource(palette_resource: &Variant) -> Result<Palette, String> {
    let quake_palette = match Instance::<QuakePalette>::from_variant(&palette_resource) {
        Ok(instance) => instance,
        Err(err) => return Err(err.to_string()),
    };

    let (owner, _) = quake_palette.decouple();
    let path = owner.get_path();
    let path = ProjectSettings::godot_singleton().globalize_path(path);
    load_palette_file(&path)
}

pub fn load_palette_file(palette_file: &GodotString) -> Result<Palette, String> {
    match quarchitect::wad::palette::read_palette(&palette_file.to_string()) {
        Ok(palette) => Ok(palette),
        Err(err) => Err(format!("Error reading palette: {:?}", err)),
    }
}

pub fn load_wad_resource(
    wad_resource: &Variant,
    palette: Option<Palette>,
) -> Result<HashMap<String, TextureInfo>, String> {
    let quake_wad = match Instance::<QuakeWad>::from_variant(&wad_resource) {
        Ok(instance) => instance,
        Err(err) => return Err(err.to_string()),
    };

    let (owner, _) = quake_wad.decouple();
    let path = owner.get_path();
    let path = ProjectSettings::godot_singleton().globalize_path(path);
    load_wad_file(&path, palette)
}

pub fn load_wad_file(
    wad_file: &GodotString,
    palette: Option<Palette>,
) -> Result<HashMap<String, TextureInfo>, String> {
    godot_print!("TODO-3: Implement minimal pre-parse to extract unique texture names from map file, use as whitelist for WAD loading");
    match quarchitect::wad::read_textures(&wad_file.to_string(), None, 1) {
        Ok(texture_info) => {
            let wad_textures = texture_info
                .into_iter()
                .fold(HashMap::new(), |mut acc, next| {
                    let quarchitect_data =
                        quarchitect::Texture::new(next.mip_texture.width, next.mip_texture.height);

                    let texture_rgb = match next.into_rgb(palette) {
                        Ok(texture_rgb) => texture_rgb,
                        Err(err) => {
                            godot_print!("Error converting texture to RGB: {:?}", err);
                            return acc;
                        }
                    };

                    let gdnative_data = wad_texture_to_gdnative_texture(
                        texture_rgb.mip_texture.width,
                        texture_rgb.mip_texture.height,
                        texture_rgb.mip_data.mip0,
                    );

                    let gdnative_data = match gdnative_data.cast::<gdnative::Texture>() {
                        Some(gdnative_data) => gdnative_data,
                        None => return acc,
                    };

                    acc.insert(
                        texture_rgb.mip_texture.name,
                        TextureInfo {
                            quarchitect_data,
                            gdnative_data,
                            gdnative_extra: HashMap::new()
                        },
                    );

                    acc
                });

            Ok(wad_textures)
        }
        Err(err) => Err(format!("Error loading WAD file: {:?}", err)),
    }
}

fn wad_texture_to_gdnative_texture(
    width: u32,
    height: u32,
    data: Vec<quarchitect::wad::Color>,
) -> gdnative::ImageTexture {
    let mut rgb_arr = gdnative::ByteArray::new();
    for quarchitect::wad::Color(r, g, b) in data {
        rgb_arr.push(r);
        rgb_arr.push(g);
        rgb_arr.push(b);
    }

    let mut image = gdnative::Image::new();
    image.create_from_data(
        width as i64,
        height as i64,
        false,
        gdnative::Image::FORMAT_RGB8,
        rgb_arr,
    );

    match image.generate_mipmaps(false) {
        Ok(()) => (),
        Err(err) => {
            godot_error!("Error generating mipmaps: {:?}", err);
        }
    };

    let mut image_texture = gdnative::ImageTexture::new();
    image_texture.create_from_image(
        Some(image),
        gdnative::Texture::FLAG_REPEAT
            | gdnative::Texture::FLAG_MIPMAPS
            | gdnative::Texture::FLAG_ANISOTROPIC_FILTER,
    );
    image_texture
}

fn list_directories(
    dir: &mut Directory,
    path: GodotString,
) -> Result<Vec<GodotString>, GodotError> {
    dir.open(path)?;
    dir.list_dir_begin(true, true)?;

    let mut directories: Vec<GodotString> = Vec::new();
    loop {
        let file = dir.get_next();
        if file.is_empty() {
            break;
        }

        if dir.current_is_dir() {
            directories.push(file);
        }
    }

    Ok(directories)
}

fn list_files(dir: &mut Directory, path: GodotString) -> Result<Vec<GodotString>, GodotError> {
    dir.open(path)?;
    dir.list_dir_begin(true, true)?;

    let mut directories: Vec<GodotString> = Vec::new();
    loop {
        let file = dir.get_next();
        if file.is_empty() {
            break;
        }

        if !dir.current_is_dir() {
            directories.push(file);
        }
    }

    Ok(directories)
}
