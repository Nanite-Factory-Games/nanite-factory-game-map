use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
    sync::Mutex,
    time::Duration,
};

use bevy::{ecs::error::warn, log::{LogPlugin, tracing}, prelude::*};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin}, DefaultInspectorConfigPlugin};
// use crate::timeline::timeline;
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};
use bevy::asset::io::{
    AssetSource, AssetSourceId,
    memory::{Dir, MemoryAssetReader},
};
use serde::{ Serialize, Deserialize };
use anyhow::{Result, bail};

mod actions;
mod camera;
mod common;
mod entities;
mod protocol;
mod selection;
mod tilemap;

#[derive(bevy::ecs::resource::Resource, Clone, Serialize, Deserialize, Debug)]
pub struct MapConfiguration {
    pub controls_enabled: bool,
    /// The id of the character entity to follow
    pub follow_id: Option<u64>,
}

impl MapConfiguration {
    pub fn new(
        controls_enabled: bool,
        follow_id: Option<u64>
    ) -> MapConfiguration {
        MapConfiguration {
            controls_enabled,
            follow_id,
        }
    }
}

pub struct ServerInfo{
    pub url: String,
    pub token: Option<String>
}

fn build(app: &mut App) {
    app
        .add_plugins(actions::build)
        .add_plugins(camera::build)
        .add_plugins(common::build)
        .add_plugins(entities::build)
        .add_plugins(protocol::build)
        .add_plugins(selection::build)
        .add_plugins(tilemap::build);
}

pub fn configure(
    configuration: MapConfiguration,
    canvas_id: Option<String>
) -> App {
    
    let mut app = App::new();

    // Create the window
    app.add_plugins(
        DefaultPlugins
            // .set(LogPlugin {
            //     filter: "info".into(),
            //     level: bevy::log::Level::INFO,
            //     custom_layer: |_app| None,
            //     fmt_layer: None
            // })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: canvas_id,
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.set_error_handler(warn);
    // app.add_plugins(EguiPlugin::default());
    // app.register_type::<CharacterEntity>();
    // app.register_type::<Sprite>();
    // app.add_plugins(FilterQueryInspectorPlugin::<(With<CharacterEntity>, With<Transform>)>::new());
    // app.add_plugins(FilterQueryInspectorPlugin::<(With<Sprite>, With<Transform>)>::new());

    build(&mut app);
    return app;
}

/// Entrypoint for starting the wasm app.
/// Runs the map from a configuration and don't connect to a server.
/// This is mostly useful for the title screen and testing purposes.
pub fn start_from_configuration(configuration: JsValue, canvas_id: Option<String>) -> Result<()> {
    // Setup tracing for propper logging levels
    let config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);

    // Deserialize the configuration and run the game
    let configuration = serde_wasm_bindgen::from_value::<MapConfiguration>(configuration).map_err(|e| anyhow::anyhow!("Error deserializing configuration: {:?}", e))?;
    let mut app = configure(configuration, canvas_id);
    app.run();
    Ok(())
}