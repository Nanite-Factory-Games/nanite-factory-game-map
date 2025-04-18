// This module will handle all entities such as players, enemies, projectiles, etc.


use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_tweening::TweeningPlugin;
use systems::*;

pub mod components;
mod systems;

pub fn entities(app: &mut App) {
    app
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, startup);
        // .add_systems(Update, on_tickrate_change);
}