use std::collections::HashMap;
use super::utils::string_key_map;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource)]
pub struct LoopTimelineIndex(pub usize);

#[derive(Resource, PartialEq)]
pub enum FrameType {
    Movement,
    Action
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