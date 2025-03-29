use bevy::{app::App, window::{Window, WindowPlugin}, DefaultPlugins};
use bevy::prelude::*;

mod tilemap;

pub fn register(app: &mut App) {
    app
        .add_plugins(tilemap::tilemap);
}