use bevy::prelude::*;

use crate::{MapEvent, TimelineFrame};

#[derive(Resource)]
pub struct WsSender(pub Option<ewebsock::WsSender>);

#[derive(Resource)]
pub struct FrameReceiver(pub crossbeam_channel::Receiver<TimelineFrame>);

#[derive(Resource)]
pub struct EventReceiver(pub crossbeam_channel::Receiver<MapEvent>);

#[derive(Resource)]
pub struct EventSender(pub Option<ewebsock::WsSender>);