use bevy::prelude::*;

use bevy_picking_tilemap::TilemapBackend;
use components::SelectionBoxDrawing;
use systems::*;

mod components;
mod resources;
mod systems;

pub fn selection(app: &mut App) {
    app
    // Initialize the box drawing resource
        .insert_resource(SelectionBoxDrawing::default())
        .add_systems(Startup, setup_context_modal)
        .add_systems(Update, tile_click_handler)
        .add_systems(Update, tile_down_handler)
        .add_systems(Update, tile_up_handler)
        .add_systems(Update, mouse_motion_handler)
        .add_systems(Update, draw_box_system)
        .add_plugins(TilemapBackend);
}