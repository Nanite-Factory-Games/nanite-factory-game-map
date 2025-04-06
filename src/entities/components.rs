use bevy::ecs::component::Component;


#[derive(Component)]
pub struct CharacterEntity {
    pub name: String
}

#[derive(Component)]
pub struct NPCEntity {
    pub name: String
}

#[derive(Component)]
pub struct ResourceEntity {
    pub name: String
}

#[derive(Component)]
pub struct PlayerCharacterMarker;