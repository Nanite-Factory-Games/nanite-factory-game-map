use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

use crate::client::camera::systems::on_configuration_change;

mod systems;

pub fn build(app: &mut App) {
    app
        .add_plugins(PanCamPlugin)
        .add_systems(Startup, systems::setup)
        .add_systems(FixedUpdate, on_configuration_change);
}
