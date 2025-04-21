use bevy::prelude::*;
use bevy::log::info;
use crate::{entities::components::{CharacterEntity, PlayerCharacterMarker}, FrameReceiver, LoopTimeline, LoopTimelineIndex};

use super::{FrameType, Timeline, TimelineFrame};

// We switch between movement and animation frames on every tick
pub fn alternate_frame(
    mut frame_type: ResMut<FrameType>,
) {
    *frame_type = match *frame_type {
        FrameType::Movement => FrameType::Action,
        FrameType::Action => FrameType::Movement
    }
}

// Grabs the most recent frame and sets it as the current frame
pub fn advance_timeline(
    mut timeline: ResMut<Timeline>,
    frame_type: Res<FrameType>,
    mut current_frame: ResMut<TimelineFrame>,
    loop_timeline: Res<LoopTimeline>,
    mut loop_timeline_index: ResMut<LoopTimelineIndex>
) {
    if *frame_type != FrameType::Action { return }
    if loop_timeline.0 {
        loop_timeline_index.0 += 1;
        if loop_timeline_index.0 >= timeline.0.len() {
            loop_timeline_index.0 = 0;
        }
        info!("timeline frame {}", loop_timeline_index.0);
        if let Some(frame) = timeline.0.get(loop_timeline_index.0) {
            *current_frame = frame.clone();
        }
    } else {
        let new_frame = timeline.0.pop_front().unwrap();
        *current_frame = new_frame;
    }
}

/// Grabs all frames from the timeline frame sender and adds them to the timeline
pub fn consume_timeline(
    mut timeline: ResMut<Timeline>,
    timeline_receiver: ResMut<FrameReceiver>,
) {
    timeline.0.extend(timeline_receiver.0.try_iter());
}

pub fn move_characters(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut query: Query<(&CharacterEntity, &Transform), With<PlayerCharacterMarker>>,
) {
    if *frame_type != FrameType::Movement { return } 
}

pub fn animate_characters(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut query: Query<(&CharacterEntity, &Transform), With<PlayerCharacterMarker>>,
) {
    if *frame_type != FrameType::Action { return } 


}

pub fn move_npcs(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut query: Query<(&CharacterEntity, &Transform), With<PlayerCharacterMarker>>,
) {
    if *frame_type != FrameType::Movement { return } 


}

pub fn animate_npcs(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut query: Query<(&CharacterEntity, &Transform), With<PlayerCharacterMarker>>,
) {
    if *frame_type != FrameType::Action { return } 

}
