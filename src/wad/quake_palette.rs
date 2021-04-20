use gdnative::{methods, NativeClass, Resource};

#[derive(NativeClass)]
#[user_data[gdnative::user_data::RwLockData<QuakePalette>]]
#[inherit(Resource)]
pub struct QuakePalette {}

#[methods]
impl QuakePalette {
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Quake Palette".into())
        }

        QuakePalette {}
    }
}
