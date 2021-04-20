use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods, Color,
    GodotString, NativeClass, Resource, StringArray, Variant, VariantArray, Vector3,
};

use quarchitect::game_data::forge::Metadata as QuarchitectMetadata;

#[derive(Debug, NativeClass)]
#[user_data[gdnative::user_data::RwLockData<ForgeMetadata>]]
#[register_with(register_forge_metadata)]
#[inherit(Resource)]
pub struct ForgeMetadata {
    data: QuarchitectMetadata,
}

fn register_forge_metadata(builder: &gdnative::init::ClassBuilder<ForgeMetadata>) {
    builder
        .add_property::<i32>("metadata_type")
        .with_default(0)
        .with_getter(ForgeMetadata::get_metadata_type)
        .with_setter(ForgeMetadata::set_metadata_type)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<StringArray>("base_classes")
        .with_default(StringArray::new())
        .with_getter(ForgeMetadata::get_base_classes)
        .with_setter(ForgeMetadata::set_base_classes)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Color>("color")
        .with_default(Color::rgb(1.0, 1.0, 1.0))
        .with_getter(ForgeMetadata::get_color)
        .with_setter(ForgeMetadata::set_color)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Vector3>("bounding_box_min")
        .with_default(Vector3::new(-8.0, -8.0, -8.0))
        .with_getter(ForgeMetadata::get_bounding_box_min)
        .with_setter(ForgeMetadata::set_bounding_box_min)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();

    builder
        .add_property::<Vector3>("bounding_box_max")
        .with_default(Vector3::new(8.0, 8.0, 8.0))
        .with_getter(ForgeMetadata::get_bounding_box_max)
        .with_setter(ForgeMetadata::set_bounding_box_max)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}

#[methods]
impl ForgeMetadata {
    // Getters
    pub fn get_metadata_type(&self, _owner: Resource) -> i32 {
        match &self.data {
            QuarchitectMetadata::Base(_) => 0,
            QuarchitectMetadata::Color(_) => 1,
            QuarchitectMetadata::Size(_, _) => 2,
        }
    }

    pub fn get_base_classes(&self, _owner: Resource) -> StringArray {
        match &self.data {
            QuarchitectMetadata::Base(base_classes) => {
                let mut string_array = StringArray::new();
                for base_class in base_classes {
                    string_array.push(&GodotString::from_str(&base_class));
                }
                string_array
            }
            QuarchitectMetadata::Color(_) => StringArray::new(),
            QuarchitectMetadata::Size(_, _) => StringArray::new(),
        }
    }

    pub fn get_color(&self, _owner: Resource) -> Color {
        match &self.data {
            QuarchitectMetadata::Base(_) => Color::rgb(1.0, 1.0, 1.0),
            QuarchitectMetadata::Color(color) => Color::rgb(color.r, color.g, color.b),
            QuarchitectMetadata::Size(_, _) => Color::rgb(1.0, 1.0, 1.0),
        }
    }

    pub fn get_bounding_box_min(&self, _owner: Resource) -> Vector3 {
        match &self.data {
            QuarchitectMetadata::Base(_) => Vector3::default(),
            QuarchitectMetadata::Color(_) => Vector3::default(),
            QuarchitectMetadata::Size(min, _) => Vector3::new(min.x(), min.y(), min.z()),
        }
    }

    pub fn get_bounding_box_max(&self, _owner: Resource) -> Vector3 {
        match &self.data {
            QuarchitectMetadata::Base(_) => Vector3::default(),
            QuarchitectMetadata::Color(_) => Vector3::default(),
            QuarchitectMetadata::Size(_, max) => Vector3::new(max.x(), max.y(), max.z()),
        }
    }

    // Setters
    pub fn set_metadata_type(&mut self, mut owner: Resource, new_metadata_type: i32) {
        let metadata_type: i32 = self.data.clone().into();
        if metadata_type == new_metadata_type {
            return;
        }

        match new_metadata_type {
            0 => {
                self.data = QuarchitectMetadata::Base(Vec::default());
                owner.set_name("Base".into());
            }
            1 => {
                self.data = QuarchitectMetadata::Color(quarchitect::Color::new(1.0, 1.0, 1.0));
                owner.set_name("Color".into());
            }
            2 => {
                self.data = QuarchitectMetadata::Size(
                    quarchitect::Vector3::default(),
                    quarchitect::Vector3::default(),
                );
                owner.set_name("Size".into());
            }
            _ => panic!("Unexpected metadata type"),
        }

        unsafe {
            owner.property_list_changed_notify();
        }
    }

    pub fn set_base_classes(&mut self, _owner: Resource, new_base_classes: StringArray) {
        let mut vec: Vec<String> = Vec::new();
        for i in 0..new_base_classes.len() {
            vec.push(new_base_classes.get(i).to_string());
        }
        self.data = QuarchitectMetadata::Base(vec)
    }

    pub fn set_color(&mut self, _owner: Resource, new_color: Color) {
        self.data = QuarchitectMetadata::Color(quarchitect::Color::new(
            new_color.r,
            new_color.g,
            new_color.b,
        ))
    }

    pub fn set_bounding_box_min(&mut self, _owner: Resource, new_bounding_box_min: Vector3) {
        let max = match self.data {
            QuarchitectMetadata::Size(_, max) => max,
            _ => quarchitect::Vector3::default(),
        };

        self.data = QuarchitectMetadata::Size(
            quarchitect::Vector3::new(
                new_bounding_box_min.x,
                new_bounding_box_min.y,
                new_bounding_box_min.z,
            ),
            max,
        )
    }

    pub fn set_bounding_box_max(&mut self, _owner: Resource, new_bounding_box_max: Vector3) {
        let min = match self.data {
            QuarchitectMetadata::Size(min, _) => min,
            _ => quarchitect::Vector3::default(),
        };

        self.data = QuarchitectMetadata::Size(
            min,
            quarchitect::Vector3::new(
                new_bounding_box_max.x,
                new_bounding_box_max.y,
                new_bounding_box_max.z,
            ),
        )
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Metadata".into())
        }

        let data = QuarchitectMetadata::base(Vec::new());

        ForgeMetadata { data }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Forge Metadata",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "metadata_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Base,Color,Size"),
                None,
            ),
        ));

        match self.data {
            QuarchitectMetadata::Base(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "base_classes",
                        gdnative::GlobalConstants::TYPE_STRING_ARRAY,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            QuarchitectMetadata::Color(_) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "color",
                        gdnative::GlobalConstants::TYPE_COLOR,
                        None,
                        None,
                        None,
                    ),
                ));
            }
            QuarchitectMetadata::Size(_, _) => {
                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "bounding_box_min",
                        gdnative::GlobalConstants::TYPE_VECTOR3,
                        None,
                        None,
                        None,
                    ),
                ));

                property_list.push(&Variant::from_dictionary(
                    &crate::util::build_property_dictionary(
                        "bounding_box_max",
                        gdnative::GlobalConstants::TYPE_VECTOR3,
                        None,
                        None,
                        None,
                    ),
                ));
            }
        }

        property_list
    }

    // Business Logic
    pub fn inner(&self) -> QuarchitectMetadata {
        self.data.clone()
    }
}
