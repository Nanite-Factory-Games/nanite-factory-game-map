use bevy::{ecs::component::Component, reflect::Reflect};


#[derive(Component, Reflect)]
pub struct CharacterEntity {
    pub name: String,
}

#[derive(Component)]
pub struct NPCEntity {
    pub name: String,
    pub id: u64,
}

#[derive(Component)]
pub struct ResourceEntity {
    pub name: String
}

/// Marker to allow selecting characters that belong to the player
#[derive(Component)]
pub struct PlayerCharacterMarker;