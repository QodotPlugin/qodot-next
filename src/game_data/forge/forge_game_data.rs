use crate::game_data::forge::ForgeEntity;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
    user_data::RwLockData, FromVariant, GodotString, Instance, Map, NativeClass, Resource,
    StringArray, Variant, VariantArray,
};

use quarchitect::game_data::forge::GameData as QuarchitectForgeGameData;

#[derive(NativeClass)]
#[inherit(Resource)]
#[user_data(RwLockData<ForgeGameData>)]
#[register_with(register_forge_game_data)]
pub struct ForgeGameData {
    data: QuarchitectForgeGameData,
    pub entities: VariantArray,
    save_as: GodotString,
}

fn register_forge_game_data(builder: &gdnative::init::ClassBuilder<ForgeGameData>) {
    builder
        .add_property::<GodotString>("name")
        .with_default(GodotString::default())
        .with_getter(ForgeGameData::get_name)
        .with_setter(ForgeGameData::set_name)
        .done();

    builder
        .add_property::<StringArray>("includes")
        .with_default(StringArray::default())
        .with_getter(ForgeGameData::get_includes)
        .with_setter(ForgeGameData::set_includes)
        .done();

    builder
        .add_property::<VariantArray>("entities")
        .with_default(VariantArray::new())
        .with_ref_getter(ForgeGameData::get_entities)
        .with_setter(ForgeGameData::set_entities)
        .done();

    builder
        .add_property::<GodotString>("save_as")
        .with_default(GodotString::new())
        .with_ref_getter(ForgeGameData::get_save_as)
        .with_setter(ForgeGameData::set_save_as)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}

#[methods]
impl ForgeGameData {
    // Getters
    fn get_name(&self, _owner: Resource) -> GodotString {
        self.data.name.clone().into()
    }

    fn get_includes(&self, _owner: Resource) -> StringArray {
        let mut includes = StringArray::new();
        for include in &self.data.includes {
            includes.push(&GodotString::from_str(include));
        }
        includes
    }

    fn get_entities(&self, _owner: Resource) -> &VariantArray {
        &self.entities
    }

    fn get_save_as(&self, _owner: Resource) -> &GodotString {
        &self.save_as
    }

    // Setters
    fn set_name(&mut self, mut owner: Resource, new_name: GodotString) {
        let new_name_string = new_name.to_string();
        if self.data.name != new_name_string {
            self.data.name = new_name_string;
            owner.set_name(new_name);
        }
    }

    fn set_includes(&mut self, _owner: Resource, new_includes: StringArray) {
        let mut includes = Vec::new();
        for i in 0..new_includes.len() {
            includes.push(new_includes.get(i).to_string());
        }
        self.data.includes = includes
    }

    fn set_entities(&mut self, _owner: Resource, new_entities: VariantArray) {
        self.entities.resize(new_entities.len());
        for (i, entity) in new_entities.iter().enumerate() {
            let entity = {
                match Instance::<ForgeEntity>::from_variant(entity) {
                    Ok(_) => entity.clone(),
                    Err(_) => Variant::from_object(&Instance::<ForgeEntity>::new().into_base()),
                }
            };

            if entity != self.entities.get_val(i as i32) {
                self.entities.set(i as i32, &entity);
            }
        }
    }

    fn set_save_as(&mut self, _owner: Resource, new_save_as: GodotString) {
        if self.save_as != new_save_as {
            self.save_as = new_save_as;
        } else if !self.save_as.is_empty() {
            godot_print!("Save FGD as {:?}", self.save_as);
            let fgd = self.inner();
            let path = gdnative::ProjectSettings::godot_singleton().globalize_path(self.save_as.clone());
            match fgd.save(path.to_string()) {
                Ok(()) => (),
                Err(err) => godot_error!("Failed to save FGD to {:?}: {:?}", path, err)
            }
        }
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Forge Game Data".into())
        }

        let data = QuarchitectForgeGameData::default();
        let entities = VariantArray::new();
        let save_as = GodotString::new();

        ForgeGameData {
            data,
            entities,
            save_as,
        }
    }

    #[export]
    fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Forge Game Data",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "save_as",
                gdnative::GlobalConstants::TYPE_STRING,
                Some(36),
                Some("*.fgd"),
                None,
            ),
        ));

        property_list
    }

    // Business Logic
    pub fn inner(&self) -> QuarchitectForgeGameData {
        let definitions = self
            .entities
            .iter()
            .flat_map(
                |entity| match Instance::<ForgeEntity>::from_variant(entity) {
                    Ok(instance) => match instance.into_script().map(|script| script.inner()) {
                        Ok(entity) => Some(entity),
                        Err(err) => {
                            godot_error!("Error reading entity: {:?}", err);
                            None
                        }
                    },
                    Err(err) => {
                        godot_error!("Error reading entity: {:?}", err);
                        None
                    }
                },
            )
            .collect();

        QuarchitectForgeGameData {
            definitions,
            ..self.data.clone()
        }
    }
}
