use bevy::{picking::pointer::Location, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Event)]
pub struct TileClickEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub tile_pos: TilePos,
}

#[derive(Event)]
pub struct TileDownEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub location: Location,
    pub tile_pos: TilePos,
}

#[derive(Event)]
pub struct TileUpEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub location: Location,
    pub tile_pos: TilePos,
}