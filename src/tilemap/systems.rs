use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use super::components::*;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let map_handle = TiledMapHandle(asset_server.load("1-1.tmx"));

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}