// This module will handle all entities such as players, enemies, projectiles, etc.


use bevy::prelude::*;
use bevy_aseprite::AsepritePlugin;

mod components;
mod systems;

pub fn entities(app: &mut App) {
    app
        .add_plugins(AsepritePlugin)
        .add_systems(Startup, systems::startup);


}