use std::collections::HashMap;

use bevy::prelude::*;
use nanite_factory_game_map::timeline::resources::TimelineFrame;
use std::path::Path;

fn main() {
    // We don't want to do anything if this is targeted to wasm
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        use std::{collections::HashMap, thread};
        use bevy::prelude::*;
        use nanite_factory_game_map::{get_assets_recursively, MapConfiguration};
        // Create a hashmap of assets to the bytes of the asset for every file in the assets folder
        let mut assets = HashMap::new();
        get_assets_recursively(Path::new("assets"), &mut assets);
        let (sender, receiver) = crossbeam_channel::unbounded::<TimelineFrame>();

        let mut app = nanite_factory_game_map::configure(MapConfiguration {
            tickrate: 60,
            controls_enabled: true,
            assets,
            camera_position: Vec2::new(200., 200.),
            loop_timeline: false,
            follow_id: None,
            canvas_id: None,
        }, receiver);
        thread::spawn(move || {
            let server_tickrate = 60;
            // Walk in a square pattern indefinitely
            loop {
                for x in 10..100 {
                    let mut frame = TimelineFrame {
                        character_movements: HashMap::new(),
                        character_actions: HashMap::new(),
                        npc_movements: HashMap::new(),
                        npc_actions: HashMap::new(),
                    };
                    frame.character_movements.insert(0, Vec2::new(x as f32, 10.));
                    sender.send(frame).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000/server_tickrate));
                }
                for y in 10..100 {
                    let mut frame = TimelineFrame {
                        character_movements: HashMap::new(),
                        character_actions: HashMap::new(),
                        npc_movements: HashMap::new(),
                        npc_actions: HashMap::new(),
                    };
                    frame.character_movements.insert(0, Vec2::new(100.0, y as f32));
                    sender.send(frame).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000/server_tickrate));

                }
                for x in (10..100).rev() {
                    let mut frame = TimelineFrame {
                        character_movements: HashMap::new(),
                        character_actions: HashMap::new(),
                        npc_movements: HashMap::new(),
                        npc_actions: HashMap::new(),
                    };
                    frame.character_movements.insert(0, Vec2::new(x as f32, 100.));
                    sender.send(frame).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000/server_tickrate));
                }
                for y in (10..100).rev() {
                    let mut frame = TimelineFrame {
                        character_movements: HashMap::new(),
                        character_actions: HashMap::new(),
                        npc_movements: HashMap::new(),
                        npc_actions: HashMap::new(),
                    };
                    frame.character_movements.insert(0, Vec2::new(10.0, y as f32));
                    sender.send(frame).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000/server_tickrate));
                }
            } 
        });
        app.run();
    }
}
