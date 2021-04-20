#![allow(clippy::transmute_ptr_to_ptr)] // Silence gdnative clippy warnings

use gdnative::{
    godot_error, godot_wrap_method_inner,
    godot_wrap_method_parameter_count, init::ClassBuilder, NativeClass, Resource,
};

#[derive(Debug, NativeClass)]
#[user_data[gdnative::user_data::RwLockData<QuakeMap>]]
#[register_with(register_quake_map)]
#[inherit(Resource)]
pub struct QuakeMap {
    revision: i32,
}

fn register_quake_map(builder: &ClassBuilder<QuakeMap>) {
    builder
        .add_property::<i32>("revision")
        .with_default(0)
        .with_getter(QuakeMap::get_revision)
        .with_setter(QuakeMap::set_revision)
        .with_usage(gdnative::init::property::Usage::NOEDITOR)
        .done();
}

#[gdnative::methods]
impl QuakeMap {
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Quake Map".into())
        }

        let revision = 0;

        QuakeMap { revision }
    }

    #[export]
    pub fn get_revision(&self, _owner: Resource) -> i32 {
        self.revision
    }

    #[export]
    pub fn set_revision(&mut self, mut owner: Resource, new_revision: i32) {
        if self.revision != new_revision {
            self.revision = new_revision;
            unsafe {
                owner.emit_signal("changed".into(), &[]);
            }
        }
    }

    #[export]
    pub fn increment_revision(&mut self, owner: Resource) {
        self.set_revision(owner, self.revision + 1);
    }
}
