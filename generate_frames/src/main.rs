use std::{collections::{HashMap,HashSet}, fs};
use bevy::prelude::*;
use serde::Serialize;
use tiled::Loader;
use pathfinding::prelude::astar;

const NUM_FRAMES: usize = 60;

const FIRE_SPOT_PROPERTY: &str = "fire";
const FISH_SPOT_PROPERTY: &str = "fishing";

#[derive(Serialize)]
pub struct TimelineFrame {
    // List of ids of characters that moved and their new positions
    character_movements: HashMap<usize, Vec2>,
    // List of actions that characters performed on the tick before they moved
    character_actions: HashMap<usize, String>,
    // List of NPC movements performed on the tick
    npc_movements: HashMap<usize, Vec2>,
    // List of actions that NPCs performed on the tick
    npc_actions: HashMap<usize, String>
}


fn main() {
    let mut loader = Loader::new();
    let map = loader.load_tmx_map("../assets/title.tmx").unwrap();

    let mut collision_set = HashSet::new();
    let mut fish_set = HashSet::new();
    let mut fire_set = HashSet::new();
    let mut tree_set = HashSet::new();
    let mut farm_set = HashSet::new();
    let mut rock_set = HashSet::new();

    for layer in map.layers() {
        match layer.layer_type() {
            tiled::LayerType::Tiles(tile_layer) => match tile_layer {
                tiled::TileLayer::Finite(tiles) => {
                    for x in 0..map.width as i32 {
                        for y in 0..map.height as i32 {
                            if let Some(tile) = tiles.get_tile(
                                x as i32,
                                ((y as i32) - (map.height - 1) as i32).abs(),
                            ) {
                                if tile.get_tile().unwrap().collision.is_some() {
                                    // Add this coordinate to the set if tiles with collision
                                    collision_set.insert((x, y));
                                }
                                let tile_type = tile.id();
                                match tile_type {
                                    FISH_SPOT => {
                                        // Add this coordinate to the set if fish spots
                                        fish_set.insert((x, y));
                                    },
                                    FIRE_SPOT => {
                                        // Add this coordinate to the set if fire spots
                                        fire_set.insert((x, y));
                                    },
                                    TREE_SPOT => {
                                        // Add this coordinate to the set if tree spots
                                        tree_set.insert((x, y));
                                    },
                                    FARM_SPOT => {
                                        // Add this coordinate to the set if farm spots
                                        farm_set.insert((x, y));
                                    },
                                    ROCK_SPOT => {
                                        // Add this coordinate to the set if rock spots
                                        rock_set.insert((x, y));
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                tiled::TileLayer::Infinite(_) => panic!("Infinite tile layer not supported"),
            },
            tiled::LayerType::Objects(objects) => {
                for object in objects.objects() {
                    // println!("{:?}", object);
                    if let Some(_) = object.get_tile() {}
                }
            }
            tiled::LayerType::Image(_) => panic!("Image layer not supported"),
            tiled::LayerType::Group(_) => panic!("Group layer not supported"),
        }
    }

    // Now that we have all the sets we can generate an interesting set of scenarios
    let frames = create_scenario_1(&collision_set, &fish_set, &fire_set);

    let output = serde_json::to_string(&frames).unwrap();
    fs::write("frames.json", output).unwrap();

}


// Scenario where a couple characters are fishing and cooking the fish
fn create_scenario_1(
    collision_set: &HashSet<(i32, i32)>,
    fish_set: &HashSet<(i32, i32)>,
    fire_set: &HashSet<(i32, i32)>,
) -> Vec<TimelineFrame> {
    let mut frames: Vec<TimelineFrame> = Vec::new();
    let first_character_start = Vec2::new(81.0, 105.0);
    let first_character_current = first_character_start.clone();

    const GOAL: (i32, i32) = (85, 94);
    
    let successors = |&(x, y): &(i32,i32)| -> Vec<((i32, i32), i32)> {
        let mut neighbors = vec![];
        let directions = [(0,1), (1,0), (0,-1), (-1,0)];
        for (dx, dy) in directions.iter() {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && ny >= 0 && !collision_set.contains(&(nx, ny)) {
                    neighbors.push(((nx, ny), 1)); // 1 = cost
            }
        }
        neighbors
    };
    
    let heuristic = |&(x, y): &(i32, i32)| ((x - GOAL.0).abs() + (y - GOAL.1).abs()) as i32;

    let result = astar(
        &(first_character_start.x as i32, first_character_start.y as i32),
        &successors,
        heuristic,
        |&p| p == GOAL,
    ).unwrap().0;
    
    let frame_per_tile = (NUM_FRAMES as f32 / result.len() as f32) / 2.0;
    println!("Frames per tile: {}", frame_per_tile);

    for i in 0..NUM_FRAMES {
        let mut frame = TimelineFrame {
            character_movements: HashMap::new(),
            character_actions: HashMap::new(),
            npc_movements: HashMap::new(),
            npc_actions: HashMap::new(),
        };
        let mut index = (i as f32 / frame_per_tile) as usize;
        if index >= result.len() {
            index = result.len() - (index - result.len()+1);
        }
        let frame_coord = result[index];
        println!("Frame {}: {:?}", i, frame_coord);
        // Tiled wont show us the correct coordinates so we have to flip the y axis
        frame.character_movements.insert(0, Vec2::new(frame_coord.0 as f32, 511.0-frame_coord.1 as f32));
        // We want to show the fishing animation on this one when its at the spot
        if frame_coord.0 == GOAL.0 && frame_coord.1 == GOAL.1 {
            frame.character_actions.insert(0, "fish_right".to_string());
        } else if frame_coord.0 == first_character_start.x as i32 && frame_coord.1 == first_character_start.y as i32 {
            frame.character_actions.insert(0, "cook_left".to_string());
        }
        frames.push(frame);
    }

    return frames;
}