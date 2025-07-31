use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    path::Path,
    sync::Mutex,
    time::Duration,
};

use bevy::{ecs::system::command::init_resource, log::tracing, prelude::*};

use actions::actions;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin}, DefaultInspectorConfigPlugin};
use camera::camera;
use entities::entities;
use selection::selection;
use serde::Deserialize;
use shared::{resources::ControlsEnabled, shared};
use tilemap::tilemap;
use timeline::{resources::TimelineFrame, timeline};
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::{JsObject, JsValue, prelude::wasm_bindgen};

mod actions;
mod camera;
mod entities;
mod selection;
mod shared;
mod tilemap;
pub mod timeline;

use bevy::asset::io::{
    AssetSource, AssetSourceId,
    memory::{Dir, MemoryAssetReader},
};

use crate::entities::components::CharacterEntity;

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

#[derive(Resource)]
struct FrameReceiver(crossbeam_channel::Receiver<TimelineFrame>);

#[derive(Resource)]
struct Timeline(VecDeque<TimelineFrame>);

#[derive(Resource)]
pub struct LoopTimeline(pub bool);

#[derive(Deserialize)]
pub struct MapConfiguration {
    pub tickrate: u64,
    pub controls_enabled: bool,
    pub assets: HashMap<String, Vec<u8>>,
    pub camera_position: Vec2,
    /// The id of the character entity to follow
    pub follow_id: Option<u64>,
    pub canvas_id: Option<String>,
    pub loop_timeline: bool,
}

impl MapConfiguration {
    pub fn new(
        tickrate: u64,
        controls_enabled: bool,
        assets: HashMap<String, Vec<u8>>,
        follow_id: Option<u64>,
        canvas_id: Option<String>,
        loop_timeline: bool,
    ) -> MapConfiguration {
        MapConfiguration {
            tickrate,
            controls_enabled,
            assets,
            camera_position: Vec2::new(0., 0.),
            follow_id,
            canvas_id,
            loop_timeline,
        }
    }
}

#[derive(Deserialize)]
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

// Staric crossbeam channel sender
static FRAME_SENDER: Mutex<Option<crossbeam_channel::Sender<TimelineFrame>>> = Mutex::new(None);

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
