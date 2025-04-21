use bevy::ecs::system::Res;

use super::Timeline;

pub fn frame_available(
    timeline: Res<Timeline>,
) -> bool {
    return timeline.0.len() > 1
}