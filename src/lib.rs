use bevy::{app::App, window::{Window, WindowPlugin}, DefaultPlugins};
use bevy::prelude::*;

use tilemap::tilemap;
use camera::camera;

mod camera;
mod tilemap;

pub fn register(app: &mut App) {
    app
        .add_plugins(tilemap)
        .add_plugins(camera);
}