use std::{collections::HashMap, hash::Hash, path::Path, sync::Mutex, time::Duration};

use bevy::prelude::*;

use actions::actions;
use bevy_quick_response::QuickResponsePlugin;
use camera::camera;
use serde::Deserialize;
use shared::{resources::ControlsEnabled, shared};
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

use bevy::asset::io::{
        memory::{Dir, MemoryAssetReader},
        AssetSource, AssetSourceId,
    };

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

#[derive(Deserialize)]
pub struct MapConfiguration {
    pub tickrate: u64,
    pub controls_enabled: bool,
    pub assets: HashMap<String, Vec<u8>>,
    pub camera_position: Vec2,
    /// The id of the character entity to follow
    pub follow_id: Option<u64>,
    pub canvas_id: Option<String>,
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
    
    // Set up memory asset reader
    let memory_dir = MemoryDir { dir: Dir::default() };
    let reader = MemoryAssetReader {
        root: memory_dir.dir.clone(),
    };
    // Load assets into memory
    for (path, bytes) in configuration.assets.into_iter() {
        memory_dir.dir.insert_asset(Path::new(&path), bytes);
    }


    app.register_asset_source(
        AssetSourceId::from_static("memory"),
        AssetSource::build().with_reader(move || Box::new(reader.clone())),
    );
    app.insert_resource(memory_dir);

    // Create the window
    app
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Nanite Factory Game Map",
                ),
                canvas: configuration.canvas_id.clone(),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()));

    
    // Set tickrate
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(1000/(configuration.tickrate * 2))));
    // Set controls enabled
    app.insert_resource(ControlsEnabled(configuration.controls_enabled));


    register(&mut app);
    app.run();
}

/// Entrypoint for starting the wasm app
#[wasm_bindgen]
pub fn init(configuration: JsValue) {
    let configuration_deserialized: MapConfiguration = serde_wasm_bindgen::from_value(configuration).unwrap();
    run(configuration_deserialized);
}