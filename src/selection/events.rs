use bevy_ecs_tilemap::tiles::TilePos;
use bevy::prelude::*;

#[derive(Event)]
pub struct SelectionEvent {
    pub selection_start: TilePos,
    pub selection_end: TilePos,
}