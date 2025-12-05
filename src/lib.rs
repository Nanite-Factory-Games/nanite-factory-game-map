use std::collections::HashMap;
use cfg_if;
use serde::{Serialize, Deserialize};
use strum::EnumString;

cfg_if::cfg_if! {
    if #[cfg(feature = "app")] {
        pub mod app;
        mod actions;
        mod camera;
        mod entities;
        mod remote;
        mod selection;
        mod shared;
        mod tilemap;

        pub mod timeline;
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    } 
    
    #[cfg(feature = "app")]
    pub fn extend(&self, z: f32) -> bevy::prelude::Vec3 {
        bevy::prelude::Vec3::new(self.x, self.y, z)
    }
}

#[derive(Clone, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ServerInfo {
    pub url: String,
    pub token: Option<String>
}

pub type MapAssets = HashMap<String, Vec<u8>>;

#[derive(Clone, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct MapConfiguration {
    pub tickrate: u64,
    pub controls_enabled: bool,
    pub camera_position: Vec2,
    /// The id of the character entity to follow
    pub follow_id: Option<u64>,
    pub loop_timeline: bool,
}

impl MapConfiguration {
    pub fn new(
        tickrate: u64,
        controls_enabled: bool,
        follow_id: Option<u64>,
        loop_timeline: bool,
    ) -> MapConfiguration {
        MapConfiguration {
            tickrate,
            controls_enabled,
            camera_position: Vec2::new(0., 0.),
            follow_id,
            loop_timeline,
        }
    }
}

/// Events to control how the map behaves
#[derive(Clone, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum MapEvent {
    TimelineFrame(TimelineFrame),
    ClearTimeline,
    UpdateConfiguration(MapConfiguration),
    UpdateServerInfo(ServerInfo),
    UpdateAssets(MapAssets),
    ConnectionClosed,
}

#[derive(Default, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "app", derive(bevy::ecs::resource::Resource))]
pub struct TimelineFrame {
    // List of ids of characters that moved and their new positions
    pub character_movements: HashMap<u64, Vec2>,
    // List of actions that characters performed on the tick before they moved
    pub character_actions: HashMap<u64, (Action, Vec2)>,
    // List of NPC movements performed on the tick
    pub npc_movements: HashMap<u64, Vec2>,
    // List of actions that NPCs performed on the tick
    pub npc_actions: HashMap<u64, (Action, Vec2)>,
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