// This module will handle all entities such as players, enemies, projectiles, etc.


use std::collections::HashMap;

use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_tweening::TweeningPlugin;
use resources::EntityIdMap;
use systems::*;

pub mod components;
pub mod resources;
mod systems;

pub fn entities(app: &mut App) {
    app
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, startup)
        .insert_resource(EntityIdMap(HashMap::new()));
        // .add_systems(Update, on_tickrate_change);
}