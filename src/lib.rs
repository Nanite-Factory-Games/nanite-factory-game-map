use std::{collections::HashMap, hash::Hash, sync::Mutex};

use bevy::prelude::*;

use actions::actions;
use bevy_quick_response::QuickResponsePlugin;
use camera::camera;
use serde::Deserialize;
use shared::shared;
use entities::entities;
use selection::selection;
use tilemap::tilemap;
use timeline::timeline;
use wasm_bindgen::{prelude::wasm_bindgen, JsObject, JsValue};

mod actions;
mod camera;
mod shared;
mod entities;
mod selection;
mod tilemap;
mod timeline;

#[derive(Deserialize)]
pub struct MapConfiguration {
    pub tickrate: u64,
    pub controls_enabled: bool,
    pub assets: HashMap<String, Vec<u8>>,
    pub camera_position: Vec2,
    /// The id of the character entity to follow
    pub follow_id: Option<u64>,
}

#[derive(Deserialize)]
pub struct MapConfigurationUpdate {
    pub tickrate: Option<u64>,
    pub controls_enabled: Option<bool>,
    pub assets: Option<HashMap<String, Vec<u8>>>,
    pub camera_position: Option<Vec2>,
}

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

pub fn run(configuration: MapConfiguration) {
    let mut app = App::new();
    app
        // Create the window
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Nanite Factory Game Map",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()));
    register(&mut app);
    app.run();
}

/// Entrypoint for starting the wasm app
#[wasm_bindgen]
pub fn init(configuration: JsValue) {
}