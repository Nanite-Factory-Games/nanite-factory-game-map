use bevy::prelude::*;

mod components;
mod systems;

pub fn build(app: &mut App) {
    app
        .add_systems(Startup, systems::setup);
}