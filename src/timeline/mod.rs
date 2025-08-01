// This module will handle tracking and executing the timeline of the game
use bevy::{prelude::*};
use resources::{FrameType, LoopTimelineIndex, TimelineFrame};
use serde::{Deserialize, Deserializer};
use std::{collections::{HashMap, VecDeque}, time::Duration};

use run_conditions::*;
use systems::*;

pub mod resources;
mod run_conditions;
mod systems;
mod utils;


#[derive(Resource)]
pub struct Timeline(VecDeque<TimelineFrame>);

pub fn timeline(app: &mut App) {
    app
        .insert_resource(Timeline(VecDeque::new()))
        .insert_resource(TimelineFrame::default())
        .insert_resource(LoopTimelineIndex(0))
        // This is going to get flipped immediately so we set it to movement because we want to start with action
        .insert_resource(FrameType::Action)
        .add_systems(FixedUpdate, consume_timeline)
        .add_systems(FixedUpdate, alternate_frame.run_if(is_frame_available))
        .add_systems(FixedUpdate, advance_timeline.run_if(is_movement_frame).after(consume_timeline))
        .add_systems(FixedUpdate, (move_characters, move_npcs).run_if(is_movement_frame))
        .add_systems(FixedUpdate, (animate_characters, animate_npcs).run_if(is_action_frame));
}