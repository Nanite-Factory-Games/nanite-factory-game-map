use std::{
    collections::{HashMap, VecDeque},
    path::Path,
    sync::Mutex,
    time::Duration,
};
use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, warn};

use bevy::{log::tracing, prelude::*};

use crate::{MapAssets, MapConfiguration, MapEvent, TimelineFrame, actions::actions, remote::{self, remote, resources::{EventReceiver, EventSender}}, shared::shared};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin}, DefaultInspectorConfigPlugin};
use crate::camera::camera;
use crate::entities::entities;
use crate::selection::selection;
use serde::Deserialize;
use crate::tilemap::tilemap;
use crate::timeline::timeline;
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};
use bevy::asset::io::{
    AssetSource, AssetSourceId,
    memory::{Dir, MemoryAssetReader},
};
use anyhow::{Result, bail};
use wasm_result::wasm_result;

pub use bevy;

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

#[derive(Resource)]
struct Timeline(VecDeque<TimelineFrame>);

#[derive(Resource)]
pub struct ServerInfo{
    pub url: String,
    pub token: Option<String>
}

#[derive(Deserialize)]
pub struct MapAssetsUpdate(pub HashMap<String, Vec<u8>>);


fn register(app: &mut App) {
    app.add_plugins(actions)
        .add_plugins(camera)
        .add_plugins(shared)
        .add_plugins(entities)
        .add_plugins(remote)
        .add_plugins(selection)
        .add_plugins(tilemap)
        .add_plugins(timeline);
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn get_assets_recursively(path: &Path, assets: &mut HashMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            get_assets_recursively(&entry.path(), assets);
        } else {
            let path = entry.path().to_path_buf();
            let path_string = path.to_str().unwrap().to_string().replace("assets/", "");
            let bytes = std::fs::read(path).unwrap();
            assets.insert(path_string.clone(), bytes);
        }
    }
}


fn configure(
    configuration: MapConfiguration,
    assets: MapAssets,
    canvas_id: Option<String>,
    event_receiver: crossbeam_channel::Receiver<MapEvent>,
    server_sender: Option<ewebsock::WsSender>,
) -> App {
    GLOBAL_ERROR_HANDLER.set(warn).expect("The error handler can only be set once, globally.");
    
    let mut app = App::new();

    // Set up memory asset reader
    let memory_dir = MemoryDir {
        dir: Dir::default(),
    };
    let reader = MemoryAssetReader {
        root: memory_dir.dir.clone(),
    };
    // Load assets into memory
    for (path, bytes) in assets.into_iter() {
        memory_dir.dir.insert_asset(Path::new(&path), bytes);
    }
    
    // We only do in memory assets, but maybe in the future we'll load remote
    app.register_asset_source(
        AssetSourceId::from_static("memory"),
        AssetSource::build().with_reader(move || Box::new(reader.clone())),
    );
    app.insert_resource(memory_dir);

    // Set up the event receiver
    app.insert_resource(EventReceiver(event_receiver));

    // Set up the server sender
    app.insert_resource(EventSender(server_sender));

    // Set tickrate
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(
        1000 / (configuration.tickrate * 2),
    )));

    // Set up the configuration
    app.insert_resource(configuration);

    // Create the window
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: canvas_id,
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    // app.add_plugins(EguiPlugin::default());
    // app.register_type::<CharacterEntity>();
    // app.register_type::<Sprite>();
    // app.add_plugins(FilterQueryInspectorPlugin::<(With<CharacterEntity>, With<Transform>)>::new());
    // app.add_plugins(FilterQueryInspectorPlugin::<(With<Sprite>, With<Transform>)>::new());

    register(&mut app);
    return app;
}

/// Entrypoint for starting the wasm app.
/// Runs the map from a configuration and don't connect to a server.
/// This is mostly useful for the title screen and testing purposes.
pub fn start_from_configuration(configuration: JsValue, assets: JsValue, canvas_id: Option<String>) -> Result<()> {
    // Setup tracing for propper logging levels
    let config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);

    // Dummy sender and receiver for events
    let (event_tx, event_rx) = crossbeam_channel::unbounded::<MapEvent>();

    // Deserialize the configuration and run the game
    let configuration = serde_wasm_bindgen::from_value::<MapConfiguration>(configuration).map_err(|e| anyhow::anyhow!("Error deserializing configuration: {:?}", e))?;
    let assets = serde_wasm_bindgen::from_value::<MapAssets>(assets).map_err(|e| anyhow::anyhow!("Error deserializing assets: {:?}", e))?;
    let mut app = configure(configuration, assets, canvas_id, event_rx, None);
    app.run();
    Ok(())
}

pub fn submit_timeline_frame(frame: JsValue) -> Result<()> {
    let frame = serde_wasm_bindgen::from_value::<TimelineFrame>(frame)
        .map_err(|e| anyhow::anyhow!("Error deserializing timeline frame: {:?}", e))?;



    Ok(())
}

/// Gets the configuration from the server and starts the app.
pub fn start_from_server_info(url: String, token: Option<String>, canvas_id: Option<String>) -> Result<()> {
    // Initialize websocket and get the receiver for MapEvent messages
    let (event_tx, event_rx) = remote::websocket::init_websocket(url.clone(), token)?;
    // Wait for the configuration event
    let event = event_rx.recv()
        .map_err(|_e| anyhow::anyhow!("Failed to connect and receive event from: {}", &url))?; 
    let map_configuration = if let MapEvent::UpdateConfiguration(configuration) = event {
        configuration
    } else {
        bail!("Received invalid event while waiting for configuration");
    };
    // Get the assets event
    let event = event_rx.recv()
        .map_err(|e| anyhow::anyhow!("Failed to receive event: {:?}", e))?; 
    let assets = if let MapEvent::UpdateAssets(assets) = event {
        assets
    } else {
        bail!("Received invalid event while waiting for assets");
    };
    let mut app = configure(map_configuration, assets, canvas_id, event_rx, Some(event_tx));
    app.run();
    Ok(())
}