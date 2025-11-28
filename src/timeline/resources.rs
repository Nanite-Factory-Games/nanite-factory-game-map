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