use super::{BrushData, EntityType, PointData};
use crate::game_data::qodot_entity::QodotEntity;
use crate::game_data::qodot_worldspawn_layer::QodotWorldspawnLayer;
use gdnative::{
    godot_error, user_data::RwLockData, FromVariant, Instance, Map, NativeClass, Resource, Variant,
    VariantArray, GodotString,
};

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(register_qodot_game_data)]
pub struct QodotGameData {
    pub entities: VariantArray,
    pub worldspawn_layers: VariantArray,
}

fn register_qodot_game_data(builder: &gdnative::init::ClassBuilder<QodotGameData>) {
    builder
        .add_property::<GodotString>("Qodot Game Data")
        .with_default(GodotString::default())
        .with_usage(gdnative::init::PropertyUsage::CATEGORY)
        .done();
    
    builder
        .add_property::<VariantArray>("entities")
        .with_default(VariantArray::new())
        .with_ref_getter(QodotGameData::get_entities)
        .with_setter(QodotGameData::set_entities)
        .done();

    builder
        .add_property::<VariantArray>("worldspawn_layers")
        .with_default(VariantArray::new())
        .with_ref_getter(QodotGameData::get_worldspawn_layers)
        .with_setter(QodotGameData::set_worldspawn_layers)
        .done();
}

#[gdnative::methods]
impl QodotGameData {
    fn get_entities(&self, _owner: Resource) -> &VariantArray {
        &self.entities
    }

    fn get_worldspawn_layers(&self, _owner: Resource) -> &VariantArray {
        &self.worldspawn_layers
    }

    fn set_entities(&mut self, _owner: Resource, mut new_entities: VariantArray) {
        for i in 0..new_entities.len() {
            let new_entity = new_entities.get_ref(i);
            let new_entity = Instance::<QodotEntity>::from_variant(new_entity);
            if new_entity.is_err() {
                let new_entity = Instance::<QodotEntity>::new().into_base();
                let new_entity = &Variant::from_object(&new_entity);
                new_entities.set(i, new_entity);
            }
        }
        self.entities = new_entities
    }

    fn set_worldspawn_layers(&mut self, _owner: Resource, mut new_worldspawn_layers: VariantArray) {
        for i in 0..new_worldspawn_layers.len() {
            let new_worldspawn_layer = new_worldspawn_layers.get_ref(i);
            let new_worldspawn_layer =
                Instance::<QodotWorldspawnLayer>::from_variant(new_worldspawn_layer);
            if new_worldspawn_layer.is_err() {
                let new_worldspawn_layer = Instance::<QodotWorldspawnLayer>::new().into_base();
                let new_worldspawn_layer = &Variant::from_object(&new_worldspawn_layer);
                new_worldspawn_layers.set(i, new_worldspawn_layer);
            }
        }
        self.worldspawn_layers = new_worldspawn_layers
    }

    fn _init(mut owner: Resource) -> Self {
        if owner.get_name().is_empty() {
            owner.set_name("Qodot Game Data".into())
        }

        let entities = VariantArray::new();
        let worldspawn_layers = VariantArray::new();

        QodotGameData {
            entities,
            worldspawn_layers,
        }
    }

    pub fn to_quarchitect_game_data(&self) -> quarchitect::game_data::GameData {
        quarchitect::game_data::GameData {
            entities: self
                .entities
                .iter()
                .flat_map(QodotGameData::qodot_entity_to_quarchitect_entity)
                .collect(),
            worldspawn_layers: self
                .worldspawn_layers
                .iter()
                .flat_map(
                    QodotWorldspawnLayer::qodot_worldspawn_layer_to_quarchitect_worldspawn_layer,
                )
                .collect(),
        }
    }

    fn qodot_entity_to_quarchitect_entity(
        entity: &Variant,
    ) -> Option<quarchitect::game_data::Entity> {
        // Entity
        let entity: Result<Instance<QodotEntity>, gdnative::FromVariantError> =
            Instance::<QodotEntity>::from_variant(&entity);

        let entity = match entity {
            Ok(entity) => entity,
            Err(err) => {
                godot_error!("Failed to load entity data: {:?}", err);
                return None;
            }
        };

        let entity: RwLockData<QodotEntity> = entity.into_script();

        let entity_data: Result<
            (String, EntityType, Variant, Variant),
            gdnative::user_data::LockFailed,
        > = entity.map(|entity: &QodotEntity| {
            (
                entity.classname.to_string(),
                entity.entity_type,
                entity.point_data.clone(),
                entity.brush_data.clone(),
            )
        });

        let (classname, entity_type, point_data, brush_data) = match entity_data {
            Ok(entity_data) => entity_data,
            Err(err) => {
                godot_error!("Failed to read entity classname: {:?}", err);
                return None;
            }
        };

        // Point Data
        let point_data: Result<Instance<PointData>, gdnative::FromVariantError> =
            Instance::<PointData>::from_variant(&point_data);

        let point_data = match point_data {
            Ok(point_data) => point_data,
            Err(err) => {
                godot_error!("Failed to read entity point data: {:?}", err);
                return None;
            }
        };

        let (point_data_base, point_data_script) = point_data.decouple();

        let point_data: Result<
            (
                quarchitect::game_data::EntityType,
                quarchitect::game_data::ComponentType,
                quarchitect::game_data::PropertyApplicationType,
            ),
            gdnative::user_data::LockFailed,
        > = point_data_script.map(|point_data_script: &PointData| {
            let data = point_data_script.get_data(point_data_base);
            let spawn_type: quarchitect::game_data::EntityType = data.entity_type.clone();
            let component_type: quarchitect::game_data::ComponentType = data.component_type.clone();
            let property_application_type: quarchitect::game_data::PropertyApplicationType =
                data.property_application_type;

            (spawn_type, component_type, property_application_type)
        });

        let point_data = match point_data {
            Ok(point_data) => point_data,
            Err(err) => {
                godot_error!("Error reading point data: {:?}", err);
                return None;
            }
        };

        let (spawn_type, component_type, property_application_type) = point_data;

        match entity_type {
            EntityType::Placeholder => Some(quarchitect::game_data::Entity::point(
                classname,
                quarchitect::game_data::EntityType::Placeholder,
                component_type,
                property_application_type,
                quarchitect::game_data::Properties::default(),
            )),

            EntityType::Point => Some(quarchitect::game_data::Entity::point(
                classname,
                spawn_type,
                component_type,
                property_application_type,
                quarchitect::game_data::Properties::default(),
            )),

            EntityType::Brush => {
                let brush_data: Result<Instance<BrushData>, gdnative::FromVariantError> =
                    Instance::<BrushData>::from_variant(&brush_data);
                let brush_data = match brush_data {
                    Ok(brush_data) => brush_data,
                    Err(err) => {
                        godot_error!("Failed to read brush data: {:?}", err);
                        return None;
                    }
                };

                let (base, script) = brush_data.decouple();
                let brush_data: Result<
                    (
                        quarchitect::game_data::VisualType,
                        quarchitect::game_data::CollisionType,
                    ),
                    gdnative::user_data::LocalCellError,
                > = script.map(|script| {
                    let visual_type = script.get_visual_type(base.new_ref()).into();
                    let collision_type = script.get_collision_type(base.new_ref()).into();
                    (visual_type, collision_type)
                });

                let brush_data = match brush_data {
                    Ok(brush_data) => brush_data,
                    Err(err) => {
                        godot_error!("Failed to read brush data: {:?}", err);
                        return None;
                    }
                };

                let (visual_type, collision_type) = brush_data;

                Some(quarchitect::game_data::Entity::brush(
                    classname,
                    spawn_type,
                    component_type,
                    property_application_type,
                    quarchitect::game_data::Properties::default(),
                    visual_type,
                    collision_type,
                ))
            }
        }
    }
}
