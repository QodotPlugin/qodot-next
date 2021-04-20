use gdnative::{
    godot_error,
    init::property::{EnumHint, IntHint},
    FromVariant, GodotString, Instance, Map, NativeClass, Resource, Variant, VariantArray,
};

use super::{ForgeMetadata, ForgeProperty};
use quarchitect::game_data::forge::Entity as QuarchitectForgeEntity;

#[derive(NativeClass)]
#[user_data[gdnative::user_data::RwLockData<ForgeEntity>]]
#[register_with(register_forge_entity)]
#[inherit(Resource)]
pub struct ForgeEntity {
    data: QuarchitectForgeEntity,
    metadata: VariantArray,
    properties: VariantArray,
}

fn register_forge_entity(builder: &gdnative::init::ClassBuilder<ForgeEntity>) {
    builder
        .add_property::<GodotString>("Forge Entity")
        .with_default(GodotString::default())
        .with_usage(gdnative::init::PropertyUsage::CATEGORY)
        .done();
    
    builder
        .add_property::<i64>("class_type")
        .with_default(0)
        .with_getter(ForgeEntity::get_class_type)
        .with_setter(ForgeEntity::set_class_type)
        .with_hint(IntHint::Enum(EnumHint::new(vec![
            "Base".into(),
            "Point".into(),
            "Solid".into(),
        ])))
        .done();

    builder
        .add_property::<VariantArray>("metadata")
        .with_default(VariantArray::new())
        .with_ref_getter(ForgeEntity::get_metadata)
        .with_setter(ForgeEntity::set_metadata)
        .done();

    builder
        .add_property::<GodotString>("classname")
        .with_default(GodotString::new())
        .with_getter(ForgeEntity::get_classname)
        .with_setter(ForgeEntity::set_classname)
        .done();

    builder
        .add_property::<GodotString>("description")
        .with_default(GodotString::new())
        .with_getter(ForgeEntity::get_description)
        .with_setter(ForgeEntity::set_description)
        .done();

    builder
        .add_property::<VariantArray>("properties")
        .with_default(VariantArray::new())
        .with_ref_getter(ForgeEntity::get_properties)
        .with_setter(ForgeEntity::set_properties)
        .done();
}

#[gdnative::methods]
impl ForgeEntity {
    // Getters
    pub fn get_class_type(&self, _owner: Resource) -> i64 {
        self.data.class_type.into()
    }

    pub fn get_metadata(&self, _owner: Resource) -> &VariantArray {
        &self.metadata
    }

    pub fn get_classname(&self, _owner: Resource) -> GodotString {
        self.data.class_name.clone().into()
    }

    pub fn get_description(&self, _owner: Resource) -> GodotString {
        self.data.description.clone().into()
    }

    pub fn get_properties(&self, _owner: Resource) -> &VariantArray {
        &self.properties
    }

    // Setters
    pub fn set_class_type(&mut self, _owner: Resource, new_class_type: i64) {
        self.data.class_type = new_class_type.into()
    }

    pub fn set_metadata(&mut self, _owner: Resource, new_metadata: VariantArray) {
        self.metadata.resize(new_metadata.len());
        for (i, metadata) in new_metadata.iter().enumerate() {
            let metadata = {
                match Instance::<ForgeMetadata>::from_variant(metadata) {
                    Ok(_) => metadata.clone(),
                    Err(_) => Variant::from_object(&Instance::<ForgeMetadata>::new().into_base()),
                }
            };

            if metadata != self.metadata.get_val(i as i32) {
                self.metadata.set(i as i32, &metadata);
            }
        }
    }

    pub fn set_classname(&mut self, mut owner: Resource, new_classname: GodotString) {
        self.data.class_name = new_classname.to_string();
        owner.set_name(self.data.class_name.clone().into());
    }

    pub fn set_description(&mut self, _owner: Resource, new_description: GodotString) {
        self.data.description = new_description.to_string()
    }

    pub fn set_properties(&mut self, _owner: Resource, new_properties: VariantArray) {
        self.properties.resize(new_properties.len());
        for (i, property) in new_properties.iter().enumerate() {
            let property = {
                match Instance::<ForgeProperty>::from_variant(property) {
                    Ok(_) => property.clone(),
                    Err(_) => Variant::from_object(&Instance::<ForgeProperty>::new().into_base()),
                }
            };

            if property != self.properties.get_val(i as i32) {
                self.properties.set(i as i32, &property);
            }
        }
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Forge Entity".into())
        }

        let data = QuarchitectForgeEntity::default();
        let metadata = VariantArray::default();
        let properties = VariantArray::default();

        ForgeEntity {
            data,
            metadata,
            properties,
        }
    }

    // Business Logic
    pub fn inner(&self) -> QuarchitectForgeEntity {
        let metadata = self
            .metadata
            .iter()
            .flat_map(
                |metadata| match Instance::<ForgeMetadata>::from_variant(metadata) {
                    Ok(instance) => match instance.into_script().map(|script| script.inner()) {
                        Ok(metadata) => Some(metadata),
                        Err(err) => {
                            godot_error!("Error reading metadata: {:?}", err);
                            None
                        }
                    },
                    Err(err) => {
                        godot_error!("Error reading metadata: {:?}", err);
                        None
                    }
                },
            )
            .collect();

        let properties = self
            .properties
            .iter()
            .flat_map(
                |property| match Instance::<ForgeProperty>::from_variant(property) {
                    Ok(instance) => match instance.into_script().map(|script| script.inner()) {
                        Ok(property) => Some(property),
                        Err(err) => {
                            godot_error!("Error reading property: {:?}", err);
                            None
                        }
                    },
                    Err(err) => {
                        godot_error!("Error reading property: {:?}", err);
                        None
                    }
                },
            )
            .collect();

        QuarchitectForgeEntity {
            metadata,
            properties,
            ..self.data.clone()
        }
    }
}
