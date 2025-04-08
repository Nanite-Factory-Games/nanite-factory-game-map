use bevy::{ecs::{component::Component, system::Resource}, math::Vec2};
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct SelectionBoxMarker;

#[derive(Resource, Default)]
pub struct SelectionBoxDrawing {
    pub start_pos: Option<Vec2>,
    pub current_pos: Vec2,
    pub start_coords: Option<TilePos>,
}