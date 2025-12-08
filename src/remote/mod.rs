use bevy::prelude::*;

use crate::remote::{resources::WsSender, systems::process_incomming_events};

pub mod resources;
pub mod systems;
pub mod websocket;

pub fn remote(app: &mut App) {
    app
        .insert_resource(WsSender(None))
        .add_systems(Update, process_incomming_events);
}