use std::collections::HashMap;

use bevy::prelude::*;
use nanite_factory_game_map::MapConfiguration;


fn main() {
    nanite_factory_game_map::run(MapConfiguration {
        tickrate: 60,
        controls_enabled: true,
        assets: HashMap::new(),
        camera_position: Vec2::new(0., 0.),
        follow_id: None,
    });
}
