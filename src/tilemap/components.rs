use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;

use super::assets::TiledMap;

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: TiledMapHandle,
    pub storage: TiledLayersStorage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub render_settings: TilemapRenderSettings,
}


// Stores a list of tiled layers.
#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Component, Default)]
pub struct TiledMapHandle(pub Handle<TiledMap>);