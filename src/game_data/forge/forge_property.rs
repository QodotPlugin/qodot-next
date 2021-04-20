use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, FromVariant,
    GodotString, Instance, Map, NativeClass, Resource, StringArray, Variant, VariantArray,
};

use super::ForgeChoice;
use quarchitect::game_data::forge::{Property as QuarchitectProperty, PropertyData};

#[derive(NativeClass)]
#[user_data[gdnative::user_data::RwLockData<ForgeProperty>]]
#[register_with(register_forge_property)]
#[inherit(Resource)]
pub struct ForgeProperty {
    data: QuarchitectProperty,
    choices: VariantArray,
    last_modified_flag: Option<i32>,
}

fn register_forge_property(builder: &gdnative::init::ClassBuilder<ForgeProperty>) {
    builder
        .add_property::<GodotString>("name")
        .with_default(GodotString::default())
        .with_getter(ForgeProperty::get_name)
        .with_setter(ForgeProperty::set_name)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("short_description")
        .with_default(GodotString::default())
        .with_getter(ForgeProperty::get_short_description)
        .with_setter(ForgeProperty::set_short_description)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("long_description")
        .with_default(GodotString::default())
        .with_getter(ForgeProperty::get_long_description)
        .with_setter(ForgeProperty::set_long_description)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("property_type")
        .with_default(0)
        .with_getter(ForgeProperty::get_property_type)
        .with_setter(ForgeProperty::set_property_type)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("integer_default")
        .with_default(0)
        .with_getter(ForgeProperty::get_integer_default)
        .with_setter(ForgeProperty::set_integer_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<f32>("float_default")
        .with_default(0.0)
        .with_getter(ForgeProperty::get_float_default)
        .with_setter(ForgeProperty::set_float_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<gdnative::Vector3>("vector3_default")
        .with_default(gdnative::Vector3::default())
        .with_getter(ForgeProperty::get_vector3_default)
        .with_setter(ForgeProperty::set_vector3_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<GodotString>("string_default")
        .with_default(GodotString::default())
        .with_getter(ForgeProperty::get_string_default)
        .with_setter(ForgeProperty::set_string_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<gdnative::Color>("color_default")
        .with_default(gdnative::Color::rgb(1.0, 1.0, 1.0))
        .with_getter(ForgeProperty::get_color_default)
        .with_setter(ForgeProperty::set_color_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<VariantArray>("choices")
        .with_default(VariantArray::default())
        .with_ref_getter(ForgeProperty::get_choices)
        .with_setter(ForgeProperty::set_choices)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("choice_default")
        .with_default(0)
        .with_getter(ForgeProperty::get_choice_default)
        .with_setter(ForgeProperty::set_choice_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<StringArray>("flags")
        .with_default(StringArray::default())
        .with_getter(ForgeProperty::get_flags)
        .with_setter(ForgeProperty::set_flags)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<i32>("flags_default")
        .with_default(0)
        .with_getter(ForgeProperty::get_flags_default)
        .with_setter(ForgeProperty::set_flags_default)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}

#[gdnative::methods]
impl ForgeProperty {
    // Getters
    pub fn get_name(&self, _owner: Resource) -> GodotString {
        self.data.name.clone().into()
    }

    pub fn get_short_description(&self, _owner: Resource) -> GodotString {
        self.data.short_description.clone().into()
    }

    pub fn get_long_description(&self, _owner: Resource) -> GodotString {
        self.data.long_description.clone().into()
    }

    pub fn get_property_type(&self, _owner: Resource) -> i32 {
        match self.data.data {
            PropertyData::Integer(_) => 0,
            PropertyData::Float(_) => 1,
            PropertyData::Vector3(_) => 2,
            PropertyData::String(_) => 3,
            PropertyData::Color(_) => 4,
            PropertyData::Choices(_, _) => 5,
            PropertyData::Flags(_, _) => 6,
            PropertyData::TargetSource => 7,
            PropertyData::TargetDestination => 8,
        }
    }

    pub fn get_integer_default(&self, _owner: Resource) -> i32 {
        match self.data.data {
            PropertyData::Integer(integer) => integer,
            _ => 0,
        }
    }

    pub fn get_float_default(&self, _owner: Resource) -> f32 {
        match self.data.data {
            PropertyData::Float(float) => float,
            _ => 0.0,
        }
    }

    pub fn get_vector3_default(&self, _owner: Resource) -> gdnative::Vector3 {
        match self.data.data {
            PropertyData::Vector3(v) => gdnative::Vector3::new(v.x(), v.y(), v.z()),
            _ => gdnative::Vector3::default(),
        }
    }

    pub fn get_string_default(&self, _owner: Resource) -> GodotString {
        match &self.data.data {
            PropertyData::String(s) => GodotString::from_str(s),
            _ => GodotString::default(),
        }
    }

    pub fn get_color_default(&self, _owner: Resource) -> gdnative::Color {
        match self.data.data {
            PropertyData::Color(c) => gdnative::Color::rgb(c.r, c.g, c.b),
            _ => gdnative::Color::rgb(1.0, 1.0, 1.0),
        }
    }

    pub fn get_choices(&self, _owner: Resource) -> &VariantArray {
        &self.choices
    }

    pub fn get_choice_default(&self, _owner: Resource) -> i32 {
        match self.data.data {
            PropertyData::Choices(_, default) => default,
            _ => 0,
        }
    }

    pub fn get_flags(&self, _owner: Resource) -> StringArray {
        match &self.data.data {
            PropertyData::Flags(flags, _) => {
                let mut flag_array = StringArray::new();
                for flag in flags.iter() {
                    flag_array.push(&GodotString::from_str(flag));
                }
                flag_array
            }
            _ => StringArray::new(),
        }
    }

    pub fn get_flags_default(&self, _owner: Resource) -> i32 {
        match self.data.data {
            PropertyData::Flags(_, default) => default,
            _ => 0,
        }
    }

    // Setters
    pub fn set_name(&mut self, mut owner: Resource, new_name: GodotString) {
        let new_name_str = new_name.to_string();
        if self.data.name != new_name_str {
            self.data.name = new_name_str;
            owner.set_name(new_name);
        }
    }

    pub fn set_short_description(&mut self, _owner: Resource, new_short_description: GodotString) {
        self.data.short_description = new_short_description.to_string();
    }

    pub fn set_long_description(&mut self, _owner: Resource, new_long_description: GodotString) {
        self.data.long_description = new_long_description.to_string();
    }

    pub fn set_property_type(&mut self, mut owner: Resource, new_property_type: i32) {
        let new_data = match new_property_type {
            0 => PropertyData::Integer(0),
            1 => PropertyData::Float(0.0),
            2 => PropertyData::Vector3(quarchitect::Vector3::default()),
            3 => PropertyData::String(String::default()),
            4 => PropertyData::Color(quarchitect::Color::default()),
            5 => PropertyData::Choices(Vec::default(), 0),
            6 => PropertyData::Flags(Vec::default(), 0),
            7 => PropertyData::TargetSource,
            8 => PropertyData::TargetDestination,
            _ => panic!("Unexpected property type"),
        };

        if self.data.data != new_data {
            self.data.data = new_data;
            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    pub fn set_integer_default(&mut self, _owner: Resource, new_integer_default: i32) {
        self.data.data = PropertyData::Integer(new_integer_default)
    }

    pub fn set_float_default(&mut self, _owner: Resource, new_float_default: f32) {
        self.data.data = PropertyData::Float(new_float_default)
    }

    pub fn set_vector3_default(
        &mut self,
        _owner: Resource,
        new_vector3_default: gdnative::Vector3,
    ) {
        self.data.data = PropertyData::Vector3(quarchitect::Vector3::new(
            new_vector3_default.x,
            new_vector3_default.y,
            new_vector3_default.z,
        ))
    }

    pub fn set_string_default(&mut self, _owner: Resource, new_string_default: GodotString) {
        self.data.data = PropertyData::String(new_string_default.to_string())
    }

    pub fn set_color_default(&mut self, _owner: Resource, new_color_default: gdnative::Color) {
        self.data.data = PropertyData::Color(quarchitect::Color::new(
            new_color_default.r,
            new_color_default.g,
            new_color_default.b,
        ))
    }

    pub fn set_choices(&mut self, owner: Resource, new_choices: VariantArray) {
        self.choices.resize(new_choices.len());
        for (i, choice) in new_choices.iter().enumerate() {
            let i = i as i32;

            let choice = {
                match Instance::<ForgeChoice>::from_variant(choice) {
                    Ok(_) => choice.clone(),
                    Err(_) => Variant::from_object(&Instance::<ForgeChoice>::new().into_base()),
                }
            };

            let existing_choice = self.choices.get_val(i);
            if choice != existing_choice {
                if let Ok(existing_choice) = Instance::<ForgeChoice>::from_variant(&existing_choice)
                {
                    unsafe {
                        existing_choice.into_base().disconnect(
                            "changed".into(),
                            owner.cast::<gdnative::Object>(),
                            "property_list_changed_notify".into(),
                        );
                    }
                }

                self.choices.set(i, &choice);

                if let Ok(choice) = Instance::<ForgeChoice>::from_variant(&choice) {
                    unsafe {
                        match choice.into_base().connect(
                            "changed".into(),
                            owner.cast::<gdnative::Object>(),
                            "property_list_changed_notify".into(),
                            VariantArray::new(),
                            0,
                        ) {
                            Ok(()) => (),
                            Err(err) => godot_error!("Error connecting changed signal: {:?}", err),
                        }
                    }
                }
            }
        }
    }

    pub fn set_choice_default(&mut self, _owner: Resource, new_choice_default: i32) {
        let choices = match &self.data.data {
            PropertyData::Choices(choices, _) => choices.clone(),
            _ => Vec::new(),
        };

        self.data.data = PropertyData::Choices(choices, new_choice_default)
    }

    pub fn set_flags(&mut self, mut owner: Resource, new_flags: StringArray) {
        let mut flags = self.get_flags(owner.new_ref());
        if flags.len() != new_flags.len() {
            let old_len = flags.len();
            flags.resize(new_flags.len());
            for i in old_len..flags.len() {
                flags.set(i, &GodotString::from_str(" "));
            }
        }

        let mut call_notify = true;
        for i in 0..flags.len() {
            let cmp = flags
                .get(i)
                .to_string()
                .partial_cmp(&new_flags.get(i).to_string());

            if cmp != Some(std::cmp::Ordering::Equal) {
                flags.set(i, &new_flags.get(i));
                if Some(i) != self.last_modified_flag {
                    self.last_modified_flag = Some(i);
                }
                call_notify = false;
            }
        }

        if call_notify {
            unsafe {
                owner.call_deferred(GodotString::from_str("property_list_changed_notify"), &[]);
            }
        }

        let default = match self.data.data {
            PropertyData::Flags(_, default) => default,
            _ => 0,
        };
        self.data.data = PropertyData::Flags(
            {
                let mut flags = Vec::new();
                for i in 0..new_flags.len() {
                    flags.push(new_flags.get(i).to_string());
                }
                flags
            },
            default,
        )
    }

    pub fn set_flags_default(&mut self, _owner: Resource, new_flags_default: i32) {
        let flags = match &self.data.data {
            PropertyData::Flags(flags, _) => flags.clone(),
            _ => Vec::new(),
        };

        self.data.data = PropertyData::Flags(flags, new_flags_default)
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Property".into())
        }

        let data = QuarchitectProperty::default();
        let choices = VariantArray::default();
        let last_modified_flag = None;

        ForgeProperty {
            data,
            choices,
            last_modified_flag,
        }
    }

    #[export]
    fn _get_property_list(&self, owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Forge Property",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "name",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "short_description",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "long_description",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "property_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Integer,Float,Vector3,String,Color,Choices,Flags,Target Source,Target Destination"),
                None,
            ),
        ));

        match &self.data.data {
            PropertyData::Integer(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "integer_default",
                        gdnative::GlobalConstants::TYPE_INT,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            PropertyData::Float(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "float_default",
                        gdnative::GlobalConstants::TYPE_REAL,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            PropertyData::Vector3(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "vector3_default",
                        gdnative::GlobalConstants::TYPE_VECTOR3,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            PropertyData::String(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "string_default",
                        gdnative::GlobalConstants::TYPE_STRING,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            PropertyData::Color(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "color_default",
                        gdnative::GlobalConstants::TYPE_COLOR,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            PropertyData::Choices(_, _) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "choices",
                        gdnative::GlobalConstants::TYPE_ARRAY,
                        None,
                        None,
                        None,
                    ),
                ));

                let mut choices_string = String::new();
                let choices = self.get_choices(owner);
                for choice in choices.iter() {
                    let instance = Instance::<ForgeChoice>::from_variant(choice);
                    match instance {
                        Ok(instance) => {
                            let (base, script) = instance.decouple();
                            match script.map(|script| {
                                choices_string += &script.get_name(base).to_string();
                            }) {
                                Ok(()) => (),
                                Err(err) => godot_error!("Error reading choice string: {:?}", err),
                            };
                        }
                        Err(err) => godot_error!("Error reading choice: {:?}", err),
                    }
                    if Some(choice) != choices.iter().last() {
                        choices_string += ",";
                    }
                }

                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "choice_default",
                        gdnative::GlobalConstants::TYPE_INT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                        Some(&choices_string),
                        None,
                    ),
                ));
            }
            PropertyData::Flags(flags, _) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "flags",
                        gdnative::GlobalConstants::TYPE_STRING_ARRAY,
                        None,
                        None,
                        None,
                    ),
                ));

                let mut flags_string = String::new();
                for flag in flags {
                    flags_string += &flag;
                    if Some(flag) != flags.last() {
                        flags_string += ",";
                    }
                }

                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "flags_default",
                        gdnative::GlobalConstants::TYPE_INT,
                        Some(gdnative::GlobalConstants::PROPERTY_HINT_FLAGS),
                        Some(&flags_string),
                        None,
                    ),
                ));
            }
            PropertyData::TargetSource => (),
            PropertyData::TargetDestination => (),
        }

        property_list
    }

    // Business logic
    pub fn inner(&self) -> QuarchitectProperty {
        match self.data.data {
            PropertyData::Choices(_, default) => {
                let data = PropertyData::Choices(
                    {
                        let mut choices = Vec::new();
                        for choice in self.choices.iter() {
                            let instance = Instance::<ForgeChoice>::from_variant(choice);
                            match instance {
                                Ok(instance) => {
                                    let script = instance.into_script();
                                    match script.map(|script| {
                                        choices.push(script.inner())
                                    }) {
                                        Ok(()) => (),
                                        Err(err) => {
                                            godot_error!("Error reading choice string: {:?}", err)
                                        }
                                    };
                                }
                                Err(err) => godot_error!("Error reading choice: {:?}", err),
                            }
                        }
                        choices
                    },
                    default,
                );
                QuarchitectProperty { data, ..self.data.clone() }
            }
            _ => self.data.clone(),
        }
    }
}
