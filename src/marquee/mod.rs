use bevy::prelude::*;
use resources::SelectedEntities;
use systems::*;

mod components;
mod resources;
mod systems;


const ENTITY_SIZE_IN_PIXELS: f32 = 64.0;
const ENTITY_SIZE_IN_METERS: f32 = 1.0;

pub fn marquee(app: &mut App) {
    app
        .insert_resource(SelectedEntities::default())
        .add_systems(Startup, setup)
        // .add_systems(Update, handle_collision_events);
        .add_systems(Update, mouse_input_handler)
        .add_systems(Update, draw_marquee_selection)
        .add_systems(Update, mouse_motion_handler);
}