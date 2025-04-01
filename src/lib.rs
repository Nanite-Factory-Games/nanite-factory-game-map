use bevy::prelude::*;

use actions::actions;
use camera::camera;
use shared::shared;
use entities::entities;
use selection::selection;
use tilemap::tilemap;
use timeline::timeline;

mod actions;
mod camera;
mod shared;
mod entities;
mod selection;
mod tilemap;
mod timeline;

pub fn register(app: &mut App) {
    app
        .add_plugins(actions)
        .add_plugins(camera)
        .add_plugins(shared)
        .add_plugins(entities)
        .add_plugins(selection)
        .add_plugins(tilemap)
        .add_plugins(timeline);
}