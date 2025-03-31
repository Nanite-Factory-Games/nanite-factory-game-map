use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

mod systems;

pub fn camera(app: &mut App) {
    app
        .add_plugins(PanCamPlugin::default())
        .add_systems(Startup, systems::setup);
}
