use bevy::prelude::*;

#[derive(Event)]
pub struct TileClickEvent {
    pub entity: Entity,
    pub button: PointerButton,
}