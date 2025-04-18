use bevy::prelude::*;

use components::SelectionBoxDrawing;
use resources::SelectionMode;
use systems::*;
use events::*;

mod components;
mod events;
mod resources;
mod systems;

pub fn selection(app: &mut App) {
    app
        // Initialize the box drawing resource
        .insert_resource(SelectionBoxDrawing::default())
        .insert_resource(SelectionMode::PlayerCharacter)
        .add_event::<SelectionEvent>()
        .add_systems(Update, tile_click_handler)
        .add_systems(Update, tile_down_handler)
        .add_systems(Update, tile_up_handler)
        .add_systems(Update, draw_box_system);
}