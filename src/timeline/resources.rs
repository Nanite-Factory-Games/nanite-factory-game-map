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
    pub character_movements: HashMap<u64, Vec2>,
    // List of actions that characters performed on the tick before they moved
    #[serde(deserialize_with = "string_key_map")]
    pub character_actions: HashMap<u64, String>,
    // List of NPC movements performed on the tick
    #[serde(deserialize_with = "string_key_map")]
    pub npc_movements: HashMap<u64, Vec2>,
    // List of actions that NPCs performed on the tick
    #[serde(deserialize_with = "string_key_map")]
    pub npc_actions: HashMap<u64, String>
}