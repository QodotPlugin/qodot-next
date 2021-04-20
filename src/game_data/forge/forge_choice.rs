use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods, GodotString,
    NativeClass, Resource, Variant, VariantArray,
};

use quarchitect::game_data::forge::{Choice as QuarchitectChoice, ChoiceData};

#[derive(Debug, NativeClass)]
#[user_data[gdnative::user_data::RwLockData<ForgeChoice>]]
#[register_with(register_forge_choice)]
#[inherit(Resource)]
pub struct ForgeChoice {
    data: QuarchitectChoice,
}

fn register_forge_choice(builder: &gdnative::init::ClassBuilder<ForgeChoice>) {
    builder
        .add_property::<GodotString>("name")
        .with_default(GodotString::new())
        .with_getter(ForgeChoice::get_name)
        .with_setter(ForgeChoice::set_name)
        .done();

    builder
        .add_property::<i32>("choice_type")
        .with_default(0)
        .with_getter(ForgeChoice::get_choice_type)
        .with_setter(ForgeChoice::set_choice_type)
        .with_hint(gdnative::init::property::IntHint::Enum(
            gdnative::init::property::EnumHint::new(vec![
                "Integer".into(),
                "Float".into(),
                "String".into(),
            ]),
        ))
        .done();

    builder
        .add_property::<i32>("integer_value")
        .with_default(0)
        .with_getter(ForgeChoice::get_integer_value)
        .with_setter(ForgeChoice::set_integer_value)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<f32>("float_value")
        .with_default(0.0)
        .with_getter(ForgeChoice::get_float_value)
        .with_setter(ForgeChoice::set_float_value)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("string_value")
        .with_default(GodotString::new())
        .with_getter(ForgeChoice::get_string_value)
        .with_setter(ForgeChoice::set_string_value)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}

#[methods]
impl ForgeChoice {
    // Getters
    pub fn get_name(&self, _owner: Resource) -> GodotString {
        self.data.name.clone().into()
    }

    pub fn get_choice_type(&self, _owner: Resource) -> i32 {
        match self.data.value {
            ChoiceData::Integer(_) => 0,
            ChoiceData::Float(_) => 1,
            ChoiceData::String(_) => 2,
        }
    }

    pub fn get_integer_value(&self, _owner: Resource) -> i32 {
        match self.data.value {
            ChoiceData::Integer(integer) => integer,
            _ => 0,
        }
    }

    pub fn get_float_value(&self, _owner: Resource) -> f32 {
        match self.data.value {
            ChoiceData::Float(float) => float,
            _ => 0.0,
        }
    }

    pub fn get_string_value(&self, _owner: Resource) -> GodotString {
        match &self.data.value {
            ChoiceData::String(string) => string.into(),
            _ => GodotString::new(),
        }
    }

    // Setters
    pub fn set_name(&mut self, mut owner: Resource, new_name: GodotString) {
        let new_name = new_name.to_string();
        if self.data.name != new_name {
            self.data.name = new_name.clone();
            owner.set_name(new_name.into());
        } else {
            unsafe {
                owner.emit_signal("changed".into(), &[]);
            }
        }
    }

    pub fn set_choice_type(&mut self, mut owner: Resource, new_choice_type: i32) {
        let new_choice_type = match new_choice_type {
            0 => ChoiceData::Integer(0),
            1 => ChoiceData::Float(0.0),
            2 => ChoiceData::String(String::new()),
            _ => panic!("Unexpected choice type"),
        };

        if self.data.value != new_choice_type {
            self.data.value = new_choice_type;
            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    pub fn set_integer_value(&mut self, _owner: Resource, new_integer_value: i32) {
        self.data.value = ChoiceData::Integer(new_integer_value)
    }

    pub fn set_float_value(&mut self, _owner: Resource, new_float_value: f32) {
        self.data.value = ChoiceData::Float(new_float_value)
    }

    pub fn set_string_value(&mut self, _owner: Resource, new_string_value: GodotString) {
        self.data.value = ChoiceData::String(new_string_value.to_string())
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Choice".into())
        }

        let data = QuarchitectChoice::default();

        ForgeChoice { data }
    }

    #[export]
    fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Forge Choice",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        match self.data.value {
            ChoiceData::Integer(_) => property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "integer_value",
                    gdnative::GlobalConstants::TYPE_INT,
                    None,
                    None,
                    None,
                ),
            )),
            ChoiceData::Float(_) => property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "float_value",
                    gdnative::GlobalConstants::TYPE_REAL,
                    None,
                    None,
                    None,
                ),
            )),
            ChoiceData::String(_) => property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "string_value",
                    gdnative::GlobalConstants::TYPE_STRING,
                    None,
                    None,
                    None,
                ),
            )),
        }

        property_list
    }

    // Business Logic
    pub fn inner(&self) -> QuarchitectChoice {
        self.data.clone()
    }
}
