use bevy::ecs::system::{Res, ResMut};

use super::{Timeline, TimelineFrame};

pub fn frame_available(
    timeline: Res<Timeline>,
) -> bool {
    return timeline.0.len() > 1
}