use bevy::prelude::*;
use crate::entities::components::{CharacterEntity, PlayerCharacterMarker};

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
    mut current_frame: ResMut<TimelineFrame>
) {
    if *frame_type != FrameType::Action { return }
    let new_frame = timeline.0.pop_front().unwrap();
    *current_frame = new_frame;
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
