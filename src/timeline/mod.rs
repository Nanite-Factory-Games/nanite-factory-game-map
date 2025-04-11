// This module will handle tracking and executing the timeline of the game
use bevy::prelude::*;
use std::{collections::VecDeque, time::Duration};

use systems::*;

mod systems;

pub struct TimelineFrame {
    // List of ids of characters that moved and their new positions
    character_movements: Vec<(usize, Vec2)>,
    // List of actions that characters performed on the tick before they moved
    character_actions: Vec<(usize, String)>,
    // List of NPC movements performed on the tick
    npc_movements: Vec<(usize, Vec2)>,
    // List of actions that NPCs performed on the tick
    npc_actions: Vec<(usize, String)>,
}

#[derive(Resource)]
pub struct Timeline(VecDeque<TimelineFrame>);

pub fn timeline(app: &mut App) {
    app
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(1000/10)))
        .add_systems(FixedUpdate, move_characters)
        .add_systems(FixedUpdate, animate_characters)
        .add_systems(FixedUpdate, move_npcs)
        .add_systems(FixedUpdate, animate_npcs);
}