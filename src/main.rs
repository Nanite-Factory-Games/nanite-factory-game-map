use std::path::Path;
use std::collections::HashMap;


fn main() {
    // We don't want to do anything if this is targeted to wasm
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {

        use bevy::prelude::*;
        use nanite_factory_game_map::MapConfiguration;
        // Create a hashmap of assets to the bytes of the asset for every file in the assets folder
        let mut assets = HashMap::new();
        get_assets_recursively(Path::new("assets"), &mut assets);

        nanite_factory_game_map::run(MapConfiguration {
            tickrate: 10,
            controls_enabled: true,
            assets,
            camera_position: Vec2::new(0., 0.),
            follow_id: None,
            canvas_id: None,
        });
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn get_assets_recursively(path: &Path, assets: &mut HashMap<String, Vec<u8>>) {
    
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            get_assets_recursively(&entry.path(), assets);
        } else {
            let path = entry.path().to_path_buf();
            let path_string = path.to_str().unwrap().to_string().replace("assets/", "");
            let bytes = std::fs::read(path).unwrap();
            assets.insert(path_string.clone(), bytes);
            println!("assets.insert({:?})", path_string);
        }
    }
}