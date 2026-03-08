use bevy::ecs::{entity::Entity, resource::Resource};



#[derive(Resource, Default)]
pub struct CharacterIdMap(pub bimap::BiBTreeMap<Entity, u16>);