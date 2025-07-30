use std::collections::HashMap;

use bevy::prelude::*;
use nanite_factory_game_map::timeline::resources::TimelineFrame;
use std::path::Path;

fn main() {
    // We don't want to do anything if this is targeted to wasm
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        use std::collections::HashMap;
        use bevy::prelude::*;
        use nanite_factory_game_map::{get_assets_recursively, MapConfiguration};
        // Create a hashmap of assets to the bytes of the asset for every file in the assets folder
        let mut assets = HashMap::new();
        get_assets_recursively(Path::new("assets"), &mut assets);
        let (sender, receiver) = crossbeam_channel::unbounded::<TimelineFrame>();

        let mut app = nanite_factory_game_map::configure(MapConfiguration {
            tickrate: 10,
            controls_enabled: true,
            assets,
            camera_position: Vec2::new(0., 0.),
            loop_timeline: false,
            follow_id: None,
            canvas_id: None,
        }, receiver);
        app.run();
    }
}
