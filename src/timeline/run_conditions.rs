use bevy::ecs::system::Res;

use super::Timeline;

pub fn frame_available(
    timeline: Res<Timeline>,
) -> bool {
    // println!("checking if frame is available");
    if timeline.0.len() > 1 {
        return true;
    } else {
        // println!("timeline has no frames");
        return false;
    }
}