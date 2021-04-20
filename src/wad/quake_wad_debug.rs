use super::QuakeWad;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner,
    godot_wrap_method_parameter_count, init::ClassBuilder, methods, FromVariant, Instance,
    NativeClass, ProjectSettings, Resource, Variant, VariantArray, GodotString,
};

const PALETTE: &str = "P:/Personal/Rust/hello_cargo/src/test_data/palette/palette.lmp";

#[derive(NativeClass)]
#[user_data[gdnative::user_data::RwLockData<QuakeWadDebug>]]
#[register_with(register_quake_wad)]
#[inherit(Resource)]
pub struct QuakeWadDebug {
    quake_wad: Variant,
    textures: VariantArray,
}

pub fn register_quake_wad(builder: &ClassBuilder<QuakeWadDebug>) {
    builder
        .add_property::<GodotString>("Quake WAD Debug")
        .with_default(GodotString::default())
        .with_usage(gdnative::init::PropertyUsage::CATEGORY)
        .done();

    builder
        .add_property::<Option<Resource>>("quake_wad")
        .with_default(None)
        .with_getter(QuakeWadDebug::get_quake_wad)
        .with_setter(QuakeWadDebug::set_quake_wad)
        .done();

    builder
        .add_property::<VariantArray>("textures")
        .with_default(VariantArray::new())
        .with_ref_getter(QuakeWadDebug::get_textures)
        .with_setter(|_, _, _| {})
        .done();
}

#[methods]
impl QuakeWadDebug {
    fn get_quake_wad(&self, _: Resource) -> Option<Resource> {
        self.quake_wad.try_to_object::<Resource>()
    }

    fn get_textures(&self, _: Resource) -> &VariantArray {
        &self.textures
    }

    fn set_quake_wad(&mut self, owner: Resource, new_quake_wad: Option<Resource>) {
        let new_quake_wad = match new_quake_wad {
            Some(quake_wad) => Variant::from_object(&quake_wad),
            None => Variant::new(),
        };

        if self.quake_wad != new_quake_wad {
            self.quake_wad = new_quake_wad;
            self.load(owner);
        }
    }

    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Quake WAD Debug".into())
        }

        unsafe {
            owner.call_deferred("load".into(), &[]);
        }

        let quake_wad: Variant = Variant::new();
        let textures = VariantArray::new();

        QuakeWadDebug {
            quake_wad,
            textures,
        }
    }

    #[export]
    fn load(&mut self, mut owner: Resource) {
        let palette = match quarchitect::wad::palette::read_palette(PALETTE) {
            Ok(palette) => palette,
            Err(err) => {
                eprintln!("Error reading palette: {:?}", err);
                return;
            }
        };

        let quake_wad: Instance<QuakeWad> = match Instance::from_variant(&self.quake_wad) {
            Ok(quake_wad) => quake_wad,
            Err(err) => {
                godot_error!("Error loading Quake WAD: {:?}", err);
                return;
            }
        };

        let quake_wad = quake_wad.into_base();

        godot_print!("Quake WAD path: {:?}", quake_wad.get_path());
        let textures = match quarchitect::wad::read_textures(
            &ProjectSettings::godot_singleton()
                .globalize_path(quake_wad.get_path())
                .to_string(),
            None,
            1,
        ) {
            Ok(textures) => textures,
            Err(err) => {
                godot_error!("Error reading textures: {:?}", err);
                return;
            }
        };

        let mut texture_array = VariantArray::new();
        for texture in textures {
            let texture_rgb = match texture.into_rgb(Some(palette)) {
                Ok(texture_rgb) => texture_rgb,
                Err(err) => {
                    godot_print!("Error converting texture to RGB: {:?}", err);
                    return;
                }
            };

            let mip0_texture = QuakeWadDebug::get_texture(
                texture_rgb.mip_texture.width,
                texture_rgb.mip_texture.height,
                texture_rgb.mip_data.mip0,
            );
            texture_array.push(&gdnative::Variant::from_object(&mip0_texture));

            if let Some(mip_level) = texture_rgb.mip_data.mip1 {
                let mip1_texture = QuakeWadDebug::get_texture(
                    texture_rgb.mip_texture.width / 2,
                    texture_rgb.mip_texture.height / 2,
                    mip_level,
                );
                texture_array.push(&gdnative::Variant::from_object(&mip1_texture));
            }

            if let Some(mip_level) = texture_rgb.mip_data.mip2 {
                let mip2_texture = QuakeWadDebug::get_texture(
                    texture_rgb.mip_texture.width / 4,
                    texture_rgb.mip_texture.height / 4,
                    mip_level,
                );
                texture_array.push(&gdnative::Variant::from_object(&mip2_texture));
            }

            if let Some(mip_level) = texture_rgb.mip_data.mip3 {
                let mip3_texture = QuakeWadDebug::get_texture(
                    texture_rgb.mip_texture.width / 8,
                    texture_rgb.mip_texture.height / 8,
                    mip_level,
                );
                texture_array.push(&gdnative::Variant::from_object(&mip3_texture));
            }
        }

        self.textures = texture_array;
        unsafe {
            owner.property_list_changed_notify();
        }
    }

    fn get_texture(
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
}
