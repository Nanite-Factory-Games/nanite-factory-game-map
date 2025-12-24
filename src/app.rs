use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
    sync::Mutex,
    time::Duration,
};
use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, warn};

use bevy::{log::{tracing, LogPlugin}, prelude::*};

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
use crate::asset_reader::NormalizingMemoryAssetReader;
use anyhow::{Result, bail};

pub use bevy;

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

#[derive(Resource)]
struct Timeline(VecDeque<TimelineFrame>);

pub struct ServerInfo{
    pub url: String,
    pub token: Option<String>
}

#[derive(Resource)]
pub struct ServerOptInfo(pub Option<ServerInfo>);

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
    // Create normalizing readers that handle relative paths (e.g., map/../sprites/...)
    let memory_reader = NormalizingMemoryAssetReader::new(reader.clone());
    let default_reader = NormalizingMemoryAssetReader::new(reader.clone());
    
    // Register memory source with "memory" prefix
    app.register_asset_source(
        AssetSourceId::from_static("memory"),
        AssetSource::build().with_reader(move || Box::new(memory_reader.clone())),
    );
    
    // Register memory source as the default BEFORE DefaultPlugins
    // This ensures all asset loads without a prefix use memory instead of file system
    app.register_asset_source(
        AssetSourceId::default(),
        AssetSource::build().with_reader(move || Box::new(default_reader.clone())),
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

    // Set the server info to none. Configure this later to haave reconnection support
    app.insert_resource(ServerOptInfo(None));

    // Create the window
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                filter: "info".into(),
                level: bevy::log::Level::INFO,
                custom_layer: |_app| None,
            })
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
    // Initialize logging early so info! macros work before the app starts
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{fmt, EnvFilter};
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(filter)
            .init();
    }
    
    // Initialize rustls crypto provider before any TLS operations
    // With the 'ring' feature enabled in Cargo.toml, rustls will automatically
    // use ring as the crypto provider, but we explicitly set it to be safe
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install default rustls crypto provider");
    }
    
    // Initialize websocket and get the receiver for MapEvent messages
    let (event_tx, event_rx) = remote::websocket::init_websocket(url.clone(), token)?;
    info!("Initialized websocket");
    // Wait for the configuration event
    info!("Waiting for configuration event");
    let event = event_rx.recv()
        .map_err(|_e| anyhow::anyhow!("Failed to connect and receive event from: {}", &url))?; 
    info!("Received configuration event");
    let map_configuration = if let MapEvent::UpdateConfiguration(configuration) = event {
        configuration
    } else {
        bail!("Received invalid event while waiting for configuration: {:#?}", event);
    };
    info!("Waiting for assets event");
    // Get the assets event
    let event = event_rx.recv()
        .map_err(|e| anyhow::anyhow!("Failed to receive event: {:?}", e))?; 
    info!("Received assets event");
    let assets = if let MapEvent::UpdateAssets(assets) = event {
        assets
    } else {
        bail!("Received invalid event while waiting for assets");
    };
    info!("Starting app");
    let mut app = configure(map_configuration, assets, canvas_id, event_rx, Some(event_tx));
    // Set the server info so reconnections can haappen
    app.insert_resource(ServerOptInfo(Some(ServerInfo {
        url,
        token: None,
    })));
    app.run();
    Ok(())
}