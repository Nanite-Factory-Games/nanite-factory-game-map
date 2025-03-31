use bevy::prelude::*;
use bevy_picking_tilemap::TilemapBackend;

use systems::*;

mod systems;

pub fn picking(app: &mut App) {
    app
        .add_plugins(TilemapBackend)
        .add_systems(Update, tile_click_handler);
}