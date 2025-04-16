use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;
use systems::on_controls_enabled_change;

mod systems;

pub fn camera(app: &mut App) {
    app
        .add_plugins(PanCamPlugin)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, on_controls_enabled_change);
}
