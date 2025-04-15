use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

mod systems;

pub fn camera(app: &mut App) {
    app
        .add_systems(Startup, systems::setup)
        .add_plugins(PanCamPlugin);
}
