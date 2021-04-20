use gdnative::{NativeClass, Resource, GodotString};

use quarchitect::game_data::BrushData as QuarchitectBrushData;

#[derive(Debug, NativeClass)]
#[inherit(Resource)]
#[register_with(register_brush_data)]
pub struct BrushData {
    data: QuarchitectBrushData
}

fn register_brush_data(builder: &gdnative::init::ClassBuilder<BrushData>) {
    builder
        .add_property::<GodotString>("Brush Data")
        .with_default(GodotString::default())
        .with_usage(gdnative::init::PropertyUsage::CATEGORY)
        .done();

    builder
        .add_property::<i64>("visual_type")
        .with_hint(gdnative::init::property::IntHint::Enum(
            gdnative::init::property::EnumHint::new(vec!["None".into(), "Mesh".into()]),
        ))
        .with_default(0)
        .with_getter(BrushData::get_visual_type)
        .with_setter(BrushData::set_visual_type)
        .done();

    builder
        .add_property::<i64>("collision_type")
        .with_hint(gdnative::init::property::IntHint::Enum(
            gdnative::init::property::EnumHint::new(vec![
                "None".into(),
                "Convex".into(),
                "Concave".into(),
            ]),
        ))
        .with_default(0)
        .with_getter(BrushData::get_collision_type)
        .with_setter(BrushData::set_collision_type)
        .done();
}

#[gdnative::methods]
impl BrushData {
    pub fn get_visual_type(&self, _: Resource) -> i64 {
        self.data.visual_type.into()
    }

    pub fn get_collision_type(&self, _: Resource) -> i64 {
        self.data.collision_type.into()
    }

    pub fn set_visual_type(&mut self, mut _owner: Resource, new_visual_type: i64) {
        self.data.visual_type = new_visual_type.into();
    }

    pub fn set_collision_type(&mut self, mut _owner: Resource, new_collision_type: i64) {
        self.data.collision_type = new_collision_type.into();
    }

    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Brush Data".into());
        }

        let data = QuarchitectBrushData::default();

        BrushData {
            data
        }
    }
}
