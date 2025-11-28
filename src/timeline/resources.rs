use std::collections::HashMap;
use super::utils::string_key_map;
use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Resource)]
pub struct LoopTimelineIndex(pub usize);

#[derive(Resource, PartialEq)]
pub enum FrameType {
    Movement,
    Action
}

#[derive(Clone, Serialize, Deserialize, EnumString, strum::Display, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
    #[serde(rename = "fishing")]
    Fishing,
    #[serde(rename = "mining")]
    Mining,
}

impl Action {
    /// Appends _n, _e, _s, _w, or _ne ... etc to the end of the string depending on the direction
    pub fn to_string_with_direction(&self, player_direction: Vec2, target_direction: Vec2) -> String {
        let mut string = self.to_string();
        if player_direction.x > 0.0 && target_direction.x < 0.0 {
            string.push_str("_n");
        } else if player_direction.x < 0.0 && target_direction.x > 0.0 {
            string.push_str("_s");
        } else if player_direction.y > 0.0 && target_direction.y < 0.0 {
            string.push_str("_e");
        } else if player_direction.y < 0.0 && target_direction.y > 0.0 {
            string.push_str("_w");
        } else if player_direction.x > 0.0 && target_direction.x < 0.0 {
            string.push_str("_ne");
        } else if player_direction.x < 0.0 && target_direction.x > 0.0 {
            string.push_str("_se");
        } else if player_direction.y > 0.0 && target_direction.y < 0.0 {
            string.push_str("_nw");
        } else if player_direction.y < 0.0 && target_direction.y > 0.0 {
            string.push_str("_sw");
        }
        string
    }
}

#[derive(Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn extend(&self, z: f32) -> bevy::prelude::Vec3 {
        bevy::prelude::Vec3::new(self.x, self.y, z)
    }
}

#[derive(Resource, Default, Deserialize, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct TimelineFrame {
    // List of ids of characters that moved and their new positions
    #[serde(deserialize_with = "string_key_map")]
    pub character_movements: HashMap<u64, Vec2>,
    // List of actions that characters performed on the tick before they moved
    #[serde(deserialize_with = "string_key_map")]
    pub character_actions: HashMap<u64, (Action, Vec2)>,
    // List of NPC movements performed on the tick
    #[serde(deserialize_with = "string_key_map")]
    pub npc_movements: HashMap<u64, Vec2>,
    // List of actions that NPCs performed on the tick
    #[serde(deserialize_with = "string_key_map")]
    pub npc_actions: HashMap<u64, (Action, Vec2)>,
}