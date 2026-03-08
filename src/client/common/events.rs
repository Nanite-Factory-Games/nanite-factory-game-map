use bevy::{picking::pointer::Location, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Message)]
pub struct TileClickEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub tile_pos: TilePos,
}

#[derive(Message)]
pub struct TileDownEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub location: Location,
    pub tile_pos: TilePos,
}

#[derive(Message)]
pub struct TileUpEvent {
    pub entity: Entity,
    pub button: PointerButton,
    pub location: Location,
    pub tile_pos: TilePos,
}