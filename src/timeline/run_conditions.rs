use bevy::ecs::system::Res;
use bevy_inspector_egui::egui::frame;

use crate::timeline::resources::FrameType;

use super::Timeline;

pub fn is_frame_available(
    timeline: Res<Timeline>,
) -> bool {
    return timeline.0.len() > 1;
}

pub fn is_action_frame(frame_type: Res<FrameType>) -> bool {
    return *frame_type == FrameType::Action;
}

pub fn is_movement_frame(frame_type: Res<FrameType>) -> bool {
    return *frame_type == FrameType::Movement;
}