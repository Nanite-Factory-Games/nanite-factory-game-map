use bevy::prelude::*;


#[derive(Resource)]
pub enum SelectionMode {
    PlayerCharacter,
    NPC,
    Resource,
}