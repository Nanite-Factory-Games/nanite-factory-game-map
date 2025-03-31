use bevy::{app::App, window::{Window, WindowPlugin}, DefaultPlugins};
use bevy::prelude::*;

use camera::camera;
use common::common;
// use marquee::marquee;
// use picking::picking;
use selection::selection;
use tilemap::tilemap;

mod camera;
mod common;
// mod marquee;
// mod picking;
mod selection;
mod tilemap;

pub fn register(app: &mut App) {
    app
        .add_plugins(camera)
        .add_plugins(common)
        // Not working yet
        // .add_plugins(marquee)
        // .add_plugins(picking)
        .add_plugins(selection)
        .add_plugins(tilemap);
}