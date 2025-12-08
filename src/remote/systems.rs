use bevy::prelude::*;

use crate::{MapEvent, app::ServerOptInfo, remote::{resources::{EventReceiver, EventSender}, websocket::init_websocket}, timeline::Timeline};

pub fn process_incomming_events(
    mut timeline: ResMut<Timeline>,
    mut event_receiver: ResMut<EventReceiver>,
    mut event_sender: ResMut<EventSender>,
    server_opt_info: ResMut<ServerOptInfo>,

) -> Result<(), BevyError> {
    // Process events one at a time so we can safely replace the receiver if needed
    loop {
        match event_receiver.0.try_recv() {
            Ok(event) => {
                match event {
                    MapEvent::TimelineFrame(frame) => timeline.0.push_back(frame),
                    MapEvent::ConnectionClosed => {
                        // Replace the receiver and sender when connection is closed
                        if let Some(server_info) = &server_opt_info.0 {
                            let (event_sender_new, event_receiver_new) = init_websocket(server_info.url.clone(), server_info.token.clone())?;
                            *event_sender = EventSender(Some(event_sender_new));
                            *event_receiver = EventReceiver(event_receiver_new);
                            // Break early after replacing the receiver - remaining events from old receiver
                            // will be lost, but we'll process events from the new receiver in the next frame
                            break;
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                // Receiver is disconnected, this shouldn't happen normally but handle it gracefully
                break;
            }
        }
    }
    Ok(())
}