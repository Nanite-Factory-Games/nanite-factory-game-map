use assets::{TiledLoader, TiledMap};
use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;

use observers::*;
use systems::*;

mod assets;
mod components;
mod observers;
mod systems;

pub fn tilemap(app: &mut App) {
    app
        .init_asset::<TiledMap>()
        .register_asset_loader(TiledLoader)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, process_loaded_maps);
}
