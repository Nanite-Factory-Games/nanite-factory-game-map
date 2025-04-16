use std::collections::HashMap;

use bevy::prelude::*;
use nanite_factory_game_map::MapConfiguration;


fn main() {
    // Create a hashmap of assets to the bytes of the asset for every file in the assets folder
    let mut assets = HashMap::new();
    for entry in std::fs::read_dir("assets").unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let path = entry.path().to_path_buf();
        let path_string = path.to_str().unwrap().to_string().replace("assets/", "");
        let bytes = std::fs::read(path).unwrap();
        assets.insert(path_string, bytes);
    }

    nanite_factory_game_map::run(MapConfiguration {
        tickrate: 60,
        controls_enabled: true,
        assets,
        camera_position: Vec2::new(0., 0.),
        follow_id: None,
    });
}
