use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

use crate::CONTROLS_ENABLED;

mod systems;

pub fn camera(app: &mut App) {
    app
        .add_systems(Startup, systems::setup);
    let controls_enabled = *CONTROLS_ENABLED.lock().expect("Could not lock controls enabled");
    if controls_enabled {
        app.add_plugins(PanCamPlugin);
    }
}
