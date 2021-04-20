use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
    user_data::RwLockData, GodotString, NativeClass, Resource, ShaderMaterial, SpatialMaterial,
    Variant, VariantArray,
};

#[derive(Debug)]
pub enum DefaultMaterialType {
    Spatial,
    Shader,
}

#[derive(Debug, NativeClass)]
#[inherit(Resource)]
#[user_data(RwLockData<QodotMaterialData>)]
#[register_with(register_texture_metadata)]
pub struct QodotMaterialData {
    pub material_type: DefaultMaterialType,

    pub spatial_material: Variant,
    pub base_texture_spatial_param: Option<gdnative::SpatialMaterialTextureParam>,

    pub shader_material: Variant,
    pub base_texture_shader_param: GodotString,
}

fn register_texture_metadata(builder: &gdnative::init::ClassBuilder<QodotMaterialData>) {
    builder
        .add_property::<i32>("material_type")
        .with_default(0)
        .with_getter(QodotMaterialData::get_material_type)
        .with_setter(QodotMaterialData::set_material_type)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("spatial_material")
        .with_default(SpatialMaterial::new().cast::<Resource>())
        .with_getter(QodotMaterialData::get_spatial_material)
        .with_setter(QodotMaterialData::set_spatial_material)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("base_texture_spatial_param")
        .with_default(1)
        .with_getter(QodotMaterialData::get_base_texture_spatial_param)
        .with_setter(QodotMaterialData::set_base_texture_spatial_param)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("shader_material")
        .with_default(ShaderMaterial::new().cast::<Resource>())
        .with_getter(QodotMaterialData::get_shader_material)
        .with_setter(QodotMaterialData::set_shader_material)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("base_texture_shader_param")
        .with_default("albedo".into())
        .with_ref_getter(QodotMaterialData::get_base_texture_shader_param)
        .with_setter(QodotMaterialData::set_base_texture_shader_param)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}

#[methods]
impl QodotMaterialData {
    // Getters
    pub fn get_material_type(&self, _: Resource) -> i32 {
        match self.material_type {
            DefaultMaterialType::Spatial => 0,
            DefaultMaterialType::Shader => 1,
        }
    }

    pub fn get_spatial_material(&self, _: Resource) -> Option<Resource> {
        self.spatial_material.try_to_object()
    }

    pub fn get_base_texture_spatial_param(&self, _: Resource) -> i32 {
        match self.base_texture_spatial_param {
            None => 0,
            Some(gdnative::SpatialMaterialTextureParam::TextureAlbedo) => 1,
            Some(gdnative::SpatialMaterialTextureParam::TextureMetallic) => 2,
            Some(gdnative::SpatialMaterialTextureParam::TextureRoughness) => 3,
            Some(gdnative::SpatialMaterialTextureParam::TextureEmission) => 4,
            Some(gdnative::SpatialMaterialTextureParam::TextureNormal) => 5,
            Some(gdnative::SpatialMaterialTextureParam::TextureRim) => 6,
            Some(gdnative::SpatialMaterialTextureParam::TextureClearcoat) => 7,
            Some(gdnative::SpatialMaterialTextureParam::TextureFlowmap) => 8,
            Some(gdnative::SpatialMaterialTextureParam::TextureAmbientOcclusion) => 9,
            Some(gdnative::SpatialMaterialTextureParam::TextureDepth) => 10,
            Some(gdnative::SpatialMaterialTextureParam::TextureSubsurfaceScattering) => 11,
            Some(gdnative::SpatialMaterialTextureParam::TextureTransmission) => 12,
            Some(gdnative::SpatialMaterialTextureParam::TextureRefraction) => 13,
            Some(gdnative::SpatialMaterialTextureParam::TextureDetailMask) => 14,
            Some(gdnative::SpatialMaterialTextureParam::TextureDetailAlbedo) => 15,
            Some(gdnative::SpatialMaterialTextureParam::TextureDetailNormal) => 16,
            Some(gdnative::SpatialMaterialTextureParam::TextureMax) => {
                panic!("Unexpected spatial material texture param")
            }
        }
    }

    pub fn get_shader_material(&self, _: Resource) -> Option<Resource> {
        self.shader_material.try_to_object()
    }

    pub fn get_base_texture_shader_param(&self, _: Resource) -> &GodotString {
        &self.base_texture_shader_param
    }

    // Setters
    pub fn set_material_type(&mut self, mut owner: Resource, new_material_type: i32) {
        self.material_type = match new_material_type {
            0 => DefaultMaterialType::Spatial,
            1 => DefaultMaterialType::Shader,
            _ => panic!("Unexpected default material type"),
        };

        unsafe { owner.property_list_changed_notify() }
    }

    pub fn set_spatial_material(&mut self, _: Resource, new_spatial_material: Option<Resource>) {
        if let Some(resource) = new_spatial_material {
            let material_result = resource.cast::<SpatialMaterial>();

            match material_result {
                Some(material) => self.spatial_material = Variant::from_object(&material),
                None => self.spatial_material = Variant::from_object(&SpatialMaterial::new()),
            }
        } else {
            self.spatial_material = Variant::from_object(&SpatialMaterial::new());
        }
    }

    pub fn set_base_texture_spatial_param(
        &mut self,
        _: Resource,
        new_base_texture_spatial_param: i32,
    ) {
        self.base_texture_spatial_param = match new_base_texture_spatial_param {
            0 => None,
            1 => Some(gdnative::SpatialMaterialTextureParam::TextureAlbedo),
            2 => Some(gdnative::SpatialMaterialTextureParam::TextureMetallic),
            3 => Some(gdnative::SpatialMaterialTextureParam::TextureRoughness),
            4 => Some(gdnative::SpatialMaterialTextureParam::TextureEmission),
            5 => Some(gdnative::SpatialMaterialTextureParam::TextureNormal),
            6 => Some(gdnative::SpatialMaterialTextureParam::TextureRim),
            7 => Some(gdnative::SpatialMaterialTextureParam::TextureClearcoat),
            8 => Some(gdnative::SpatialMaterialTextureParam::TextureFlowmap),
            9 => Some(gdnative::SpatialMaterialTextureParam::TextureAmbientOcclusion),
            10 => Some(gdnative::SpatialMaterialTextureParam::TextureDepth),
            11 => Some(gdnative::SpatialMaterialTextureParam::TextureSubsurfaceScattering),
            12 => Some(gdnative::SpatialMaterialTextureParam::TextureTransmission),
            13 => Some(gdnative::SpatialMaterialTextureParam::TextureRefraction),
            14 => Some(gdnative::SpatialMaterialTextureParam::TextureDetailMask),
            15 => Some(gdnative::SpatialMaterialTextureParam::TextureDetailAlbedo),
            16 => Some(gdnative::SpatialMaterialTextureParam::TextureDetailNormal),
            _ => panic!("Expected spatial material texture param"),
        }
    }

    pub fn set_shader_material(&mut self, _: Resource, new_shader_material: Option<Resource>) {
        if let Some(resource) = new_shader_material {
            let material_result = resource.cast::<ShaderMaterial>();

            match material_result {
                Some(material) => self.shader_material = Variant::from_object(&material),
                None => self.shader_material = Variant::from_object(&ShaderMaterial::new()),
            }
        } else {
            self.shader_material = Variant::from_object(&ShaderMaterial::new());
        }
    }

    pub fn set_base_texture_shader_param(
        &mut self,
        _: Resource,
        new_base_texture_shader_param: GodotString,
    ) {
        self.base_texture_shader_param = new_base_texture_shader_param
    }

    // Overrides
    pub fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Material Data".into());
        }

        let material_type = DefaultMaterialType::Spatial;

        let spatial_material = Variant::from_object(&SpatialMaterial::new());
        let base_texture_spatial_param = Some(gdnative::SpatialMaterialTextureParam::TextureAlbedo);

        let shader_material = Variant::from_object(&ShaderMaterial::new());
        let base_texture_shader_param = "albedo".into();

        QodotMaterialData {
            material_type,

            spatial_material,
            base_texture_spatial_param,

            shader_material,
            base_texture_shader_param,
        }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Qodot Material Data",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "material_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Spatial,Shader"),
                None,
            ),
        ));

        match self.material_type {
            DefaultMaterialType::Spatial => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "spatial_material",
                        gdnative::GlobalConstants::TYPE_OBJECT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                        Some("SpatialMaterial"),
                        None,
                    ),
                ));
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "base_texture_spatial_param",
                        gdnative::GlobalConstants::TYPE_INT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                        Some("None,Albedo,Metallic,Roughness,Emission,Normal,Rim,Clearcoat,Flowmap,Ambient Occlusion,Depth,Subsurface Scattering,Transmission,Refraction,Detail Mask,Detail Albedo,Detail Normal"),
                        None,
                    ),
                ));
            }
            DefaultMaterialType::Shader => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "shader_material",
                        gdnative::GlobalConstants::TYPE_OBJECT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                        Some("ShaderMaterial"),
                        None,
                    ),
                ));

                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "base_texture_shader_param",
                        gdnative::GlobalConstants::TYPE_STRING,
                        None,
                        None,
                        None,
                    ),
                ));
            }
        }
        property_list
    }
}
