use bevy::prelude::*;

use bevy_picking_tilemap::TilemapBackend;
use systems::*;

mod components;
mod resources;
mod systems;

pub fn selection(app: &mut App) {
    app
        .add_systems(Update, tile_click_handler)
        .add_plugins(TilemapBackend);
}