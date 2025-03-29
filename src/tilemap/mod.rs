use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;

mod assets;
mod components;
mod plugins;
mod systems;

pub fn tilemap(app: &mut App) {
    app
        .add_plugins(TilemapPlugin)
        .add_plugins(plugins::TiledMapPlugin)
        .add_systems(Startup, systems::startup)
        .add_systems(Update, systems::movement);
}
