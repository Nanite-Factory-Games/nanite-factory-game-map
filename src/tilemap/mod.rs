
use bevy::prelude::*;

use ldtk_rust::bevy::LdtkPlugin;
use systems::*;
use bevy_ecs_tilemap::prelude::*;


mod components;
mod observers;
mod systems;

pub fn tilemap(app: &mut App) {
    app
        .add_plugins(LdtkPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, process_loaded_maps);
}
