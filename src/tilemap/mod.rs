use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;

use observers::*;
use systems::*;

mod assets;
mod components;
mod observers;
mod plugins;
mod systems;

pub fn tilemap(app: &mut App) {
    app
        .add_plugins(TilemapPlugin)
        .add_plugins(plugins::TiledMapPlugin)
        .add_systems(Startup, startup);

}
