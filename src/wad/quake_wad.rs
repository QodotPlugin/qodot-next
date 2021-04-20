use gdnative::{methods, NativeClass, Resource};

#[derive(NativeClass)]
#[user_data[gdnative::user_data::RwLockData<QuakeWad>]]
#[inherit(Resource)]
pub struct QuakeWad {}

#[methods]
impl QuakeWad {
    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Quake WAD".into())
        }

        QuakeWad {}
    }
}
