use bevy::{picking::pointer::Location, prelude::*};

#[derive(Event)]
pub struct TileClickEvent {
    pub entity: Entity,
    pub button: PointerButton
}

#[derive(Event)]
pub struct TileDownEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub location: Location
}

#[derive(Event)]
pub struct TileUpEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub location: Location
}