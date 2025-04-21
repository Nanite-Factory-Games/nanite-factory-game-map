// This module will handle tracking and executing the timeline of the game
use bevy::{prelude::*};
use serde::{Deserialize, Deserializer};
use std::{collections::{HashMap, VecDeque}, time::Duration};

use run_conditions::*;
use systems::*;

mod run_conditions;
mod systems;

#[derive(Resource, PartialEq)]
enum FrameType {
    Movement,
    Action
}

fn string_key_map<'de, D, V>(deserializer: D) -> Result<HashMap<usize, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    let string_map: HashMap<String, V> = HashMap::deserialize(deserializer)?;
    let mut int_map = HashMap::with_capacity(string_map.len());
    for (k, v) in string_map {
        let parsed_key = k.parse::<usize>().map_err(serde::de::Error::custom)?;
        int_map.insert(parsed_key, v);
    }
    Ok(int_map)
}

#[derive(Resource, Default, Deserialize, Clone)]
pub struct TimelineFrame {
    // List of ids of characters that moved and their new positions
    #[serde(deserialize_with = "string_key_map")]
    character_movements: HashMap<usize, Vec2>,
    // List of actions that characters performed on the tick before they moved
    #[serde(deserialize_with = "string_key_map")]
    character_actions: HashMap<usize, String>,
    // List of NPC movements performed on the tick
    #[serde(deserialize_with = "string_key_map")]
    npc_movements: HashMap<usize, Vec2>,
    // List of actions that NPCs performed on the tick
    #[serde(deserialize_with = "string_key_map")]
    npc_actions: HashMap<usize, String>
}

#[derive(Resource)]
pub struct Timeline(VecDeque<TimelineFrame>);

pub fn timeline(app: &mut App) {
    app
        .insert_resource(Timeline(VecDeque::new()))
        .insert_resource(TimelineFrame::default())
        // This is going to get flipped immediately so we set it to movement because we want to start with action
        .insert_resource(FrameType::Movement)
        .add_systems(Update, consume_timeline)
        .add_systems(FixedUpdate, alternate_frame.run_if(frame_available))
        .add_systems(FixedUpdate, advance_timeline.after(alternate_frame))
        .add_systems(FixedUpdate, (
                move_characters,
                animate_characters,
                move_npcs,
                animate_npcs
            )
            .run_if(frame_available)
            .after(advance_timeline)
        );
}