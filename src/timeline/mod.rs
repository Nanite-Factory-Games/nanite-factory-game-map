// This module will handle tracking and executing the timeline of the game
use bevy::{prelude::*};
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

#[derive(Resource, Default)]
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

#[derive(Resource)]
pub struct Timeline(VecDeque<TimelineFrame>);

pub fn timeline(app: &mut App) {
    app
        .insert_resource(Timeline(VecDeque::new()))
        .insert_resource(TimelineFrame::default())
        // This is going to get flipped immediately so we set it to movement because we want to start with action
        .insert_resource(FrameType::Movement)
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