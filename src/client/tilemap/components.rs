use bevy::ecs::bundle::Bundle;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use ldtk_rust::bevy::LdtkMap;


#[derive(Default, Component)]
pub struct LdtkMapConfig {
    pub selected_level: usize,
}

#[derive(Default, Component)]
pub struct LdtkMapHandle(pub Handle<LdtkMap>);

#[derive(Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_map: LdtkMapHandle,
    pub ldtk_map_config: LdtkMapConfig,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}