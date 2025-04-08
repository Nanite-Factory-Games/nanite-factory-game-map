use bevy::ecs::component::Component;


#[derive(Component)]
pub struct CharacterEntity {
    pub name: String,
    pub id: u32,
}

#[derive(Component)]
pub struct NPCEntity {
    pub name: String,
    pub id: u32,
}

#[derive(Component)]
pub struct ResourceEntity {
    pub name: String
}

/// Marker to allow selecting characters that belong to the player
#[derive(Component)]
pub struct PlayerCharacterMarker;