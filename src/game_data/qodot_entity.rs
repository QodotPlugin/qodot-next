use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods, FromVariant,
    GodotString, Instance, NativeClass, Resource, Variant, VariantArray,
};

use crate::game_data::BrushData;
use crate::game_data::PointData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityType {
    Point,
    Brush,
    Placeholder,
}

impl Into<i64> for EntityType {
    fn into(self) -> i64 {
        match self {
            EntityType::Point => 0,
            EntityType::Brush => 1,
            EntityType::Placeholder => 2,
        }
    }
}

impl From<i64> for EntityType {
    fn from(i: i64) -> Self {
        match i {
            0 => EntityType::Point,
            1 => EntityType::Brush,
            2 => EntityType::Placeholder,
            _ => panic!("Invalid entity type"),
        }
    }
}

#[derive(Debug, NativeClass)]
#[inherit(Resource)]
#[user_data(gdnative::user_data::RwLockData<QodotEntity>)]
#[register_with(register_qodot_entity)]
pub struct QodotEntity {
    pub classname: GodotString,
    pub entity_type: EntityType,
    pub point_data: Variant,
    pub brush_data: Variant,
}

fn register_qodot_entity(builder: &gdnative::init::ClassBuilder<QodotEntity>) {
    builder
        .add_property::<GodotString>("classname")
        .with_default("".into())
        .with_ref_getter(QodotEntity::get_classname)
        .with_setter(QodotEntity::set_classname)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<i64>("entity_type")
        .with_default(0)
        .with_getter(QodotEntity::get_entity_type)
        .with_setter(QodotEntity::set_entity_type)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("point_data")
        .with_default(None)
        .with_getter(QodotEntity::get_point_data)
        .with_setter(QodotEntity::set_point_data)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();

    builder
        .add_property::<Option<Resource>>("brush_data")
        .with_default(None)
        .with_getter(QodotEntity::get_brush_data)
        .with_setter(QodotEntity::set_brush_data)
        .with_usage(gdnative::init::PropertyUsage::NOEDITOR)
        .done();
}

#[methods]
impl QodotEntity {
    // Getters
    pub fn get_classname(&self, _owner: Resource) -> &GodotString {
        &self.classname
    }

    pub fn get_entity_type(&self, _owner: Resource) -> i64 {
        self.entity_type.into()
    }

    pub fn get_point_data(&self, _owner: Resource) -> Option<Resource> {
        self.point_data.try_to_object()
    }

    pub fn get_brush_data(&self, _owner: Resource) -> Option<Resource> {
        self.brush_data.try_to_object()
    }

    // Setters
    pub fn set_classname(&mut self, mut owner: Resource, new_classname: GodotString) {
        if self.classname != new_classname {
            self.classname = new_classname;
            owner.set_name(self.classname.clone());
            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    pub fn set_entity_type(&mut self, mut owner: Resource, new_entity_type: i64) {
        if self.entity_type != new_entity_type.into() {
            self.entity_type = new_entity_type.into();
            unsafe {
                owner.property_list_changed_notify();
            }
        }
    }

    pub fn set_point_data(&mut self, _owner: Resource, new_point_data: Option<Resource>) {
        let new_point_data = new_point_data.as_ref().map(|point_data: &Resource| {
            Instance::<PointData>::from_variant(&Variant::from_object(point_data))
        });

        if let Some(Ok(new_point_data)) = new_point_data {
            self.point_data = Variant::from_object(&new_point_data.into_base())
        } else {
            self.point_data = Variant::from_object(&Instance::<PointData>::new().into_base())
        }
    }

    pub fn set_brush_data(&mut self, _owner: Resource, new_brush_data: Option<Resource>) {
        let new_brush_data = new_brush_data.as_ref().map(|brush_data: &Resource| {
            Instance::<BrushData>::from_variant(&Variant::from_object(brush_data))
        });

        if let Some(Ok(new_brush_data)) = new_brush_data {
            self.brush_data = Variant::from_object(&new_brush_data.into_base())
        } else {
            self.brush_data = Variant::from_object(&Instance::<PointData>::new().into_base())
        }
    }

    // Overrides
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Qodot Entity".into())
        }

        let classname = GodotString::new();
        let entity_type = EntityType::Point;
        let point_data = Variant::from_object(&Instance::<PointData>::new().into_base());
        let brush_data = Variant::from_object(&Instance::<BrushData>::new().into_base());

        QodotEntity {
            classname,
            entity_type,
            point_data,
            brush_data,
        }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: Resource) -> VariantArray {
        let mut property_list = VariantArray::new();

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "Qodot Entity",
                gdnative::GlobalConstants::TYPE_NIL,
                None,
                None,
                Some(gdnative::GlobalConstants::PROPERTY_USAGE_CATEGORY),
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "classname",
                gdnative::GlobalConstants::TYPE_STRING,
                None,
                None,
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "entity_type",
                gdnative::GlobalConstants::TYPE_INT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_ENUM),
                Some("Point,Brush"),
                None,
            ),
        ));

        property_list.push(&Variant::from_dictionary(
            &crate::util::build_property_dictionary(
                "point_data",
                gdnative::GlobalConstants::TYPE_OBJECT,
                Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                Some("Resource"),
                None,
            ),
        ));

        if self.entity_type == EntityType::Brush {
            property_list.push(&Variant::from_dictionary(
                &crate::util::build_property_dictionary(
                    "brush_data",
                    gdnative::GlobalConstants::TYPE_OBJECT,
                    Some(gdnative::GlobalConstants::PROPERTY_HINT_RESOURCE_TYPE),
                    Some("Resource"),
                    None,
                ),
            ));
        }

        property_list
    }
}
