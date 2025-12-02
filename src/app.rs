use std::{
    collections::{HashMap, VecDeque},
    path::Path,
    sync::Mutex,
    time::Duration,
};
use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, warn};

use bevy::{log::tracing, prelude::*};

use crate::{MapConfiguration, MapEvent, TimelineFrame, actions::actions};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin}, DefaultInspectorConfigPlugin};
use crate::camera::camera;
use crate::entities::entities;
use crate::selection::selection;
use serde::Deserialize;
use crate::shared::{resources::ControlsEnabled, shared};
use crate::tilemap::tilemap;
use crate::timeline::timeline;
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use bevy::asset::io::{
    AssetSource, AssetSourceId,
    memory::{Dir, MemoryAssetReader},
};

pub use bevy;

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

#[derive(Resource)]
pub struct FrameReceiver(pub crossbeam_channel::Receiver<TimelineFrame>);

#[derive(Resource)]
struct Timeline(VecDeque<TimelineFrame>);

#[derive(Resource)]
pub struct LoopTimeline(pub bool);



#[derive(Deserialize, Resource)]
pub struct CameraConfiguration {
    pub position: Vec2,
}

#[derive(Deserialize)]
pub struct MapConfigurationUpdate {
    pub tickrate: Option<u64>,
    pub controls_enabled: Option<bool>,
    pub assets: Option<HashMap<String, Vec<u8>>>,
    pub camera_position: Option<Vec2>,
}

/// Staric crossbeam channel sender
static FRAME_SENDER: Mutex<Option<crossbeam_channel::Sender<TimelineFrame>>> = Mutex::new(None);

/// Static crossbeam channel sender for events
static EVENT_SENDER: Mutex<Option<crossbeam_channel::Sender<MapEvent>>> = Mutex::new(None);

fn register(app: &mut App) {
    app.add_plugins(actions)
        .add_plugins(camera)
        .add_plugins(shared)
        .add_plugins(entities)
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

pub fn configure(
    configuration: MapConfiguration,
    frame_receiver: crossbeam_channel::Receiver<TimelineFrame>,
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
    for (path, bytes) in configuration.assets.into_iter() {
        memory_dir.dir.insert_asset(Path::new(&path), bytes);
    }

    app.register_asset_source(
        AssetSourceId::from_static("memory"),
        AssetSource::build().with_reader(move || Box::new(reader.clone())),
    );
    app.insert_resource(memory_dir);

    // Set up the frame receiver
    let frame_receiver = FrameReceiver(frame_receiver);
    app.insert_resource(frame_receiver);

    // Set wether the timeline should loop
    app.insert_resource(LoopTimeline(configuration.loop_timeline));

    app.insert_resource(CameraConfiguration {
        position: bevy::prelude::Vec2{ x: configuration.camera_position.x, y: configuration.camera_position.y }
    });

    // Create the window
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: configuration.canvas_id.clone(),
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

    // Set tickrate
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(
        1000 / (configuration.tickrate * 2),
    )));
    // Set controls enabled
    app.insert_resource(ControlsEnabled(configuration.controls_enabled));

    register(&mut app);
    return app;
}

/// Entrypoint for starting the wasm app
#[wasm_bindgen]
pub fn start(configuration: JsValue) {
    // Setup tracing for propper logging levels
    let config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);

    // Set up the frame sender
    let (sender, receiver) = crossbeam_channel::unbounded::<TimelineFrame>();
    let mut frame_sender_lock = FRAME_SENDER.lock().unwrap();
    *frame_sender_lock = Some(sender);
    drop(frame_sender_lock);

    // Deserialize the configuration and run the game
    match serde_wasm_bindgen::from_value::<MapConfiguration>(configuration) {
        Ok(configuration) => {
            let mut app = configure(configuration, receiver);
            app.run();
        }
        Err(e) => {
            error!("Error thrown during initialization: {:?}", e);
        }
    }
}

#[wasm_bindgen]
pub fn submit_timeline_frame(frame: JsValue) {
    // info!("Submitting frame");
    match serde_wasm_bindgen::from_value::<TimelineFrame>(frame) {
        Ok(frame) => {
            // Don't freak out, this is probably not an issue
            let frame_sender_lock = FRAME_SENDER.lock().unwrap();
            frame_sender_lock
                .as_ref()
                .expect("Frame sender not initialized")
                .send(frame)
                .unwrap();
        }
        Err(e) => {
            error!("Error thrown when deserializing timeline frame: {:?}", e);
        }
    }
}

#[wasm_bindgen]
pub fn clear_timeline() {
    let event_lock = EVENT_SENDER.lock().unwrap();
    event_lock.as_ref().unwrap().send(MapEvent::ClearTimeline).unwrap();

}

#[wasm_bindgen]
pub fn update_configuration(configuration: JsValue) {
    let event_lock = EVENT_SENDER.lock().unwrap();
    let configuration = serde_wasm_bindgen::from_value::<MapConfiguration>(configuration).unwrap();
    event_lock.as_ref().unwrap().send(MapEvent::UpdateConfiguration(configuration)).unwrap();
}