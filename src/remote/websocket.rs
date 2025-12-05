use std::{cell::RefCell, ops::ControlFlow};
use bevy::log::{info, warn, error};
use ewebsock::{WsEvent, WsEvent::{Message, Error, Opened, Closed}, WsMessage::*};
use anyhow::Context;
use crossbeam_channel::{Receiver, Sender};

use crate::MapEvent;


/// Initialize the thread-local websocket connection
/// Returns a receiver for MapEvent messages received from the websocket
pub fn init_websocket(url: String, token: Option<String>) -> anyhow::Result<(ewebsock::WsSender, Receiver<MapEvent>)> {
    // Create a channel for MapEvent messages
    let (event_sender, event_receiver) = crossbeam_channel::unbounded::<MapEvent>();
    
    // Construct options for the websocket, including a token if needed for authentication
    let mut options = ewebsock::Options::default();
    if let Some(token) = token {
        options.additional_headers.insert(0, ("Authorization".to_string(), token));
    }
    // Create a closure that captures the event sender
    let sender_clone = event_sender.clone();
    
    let handle_event_closure = move |event: WsEvent| -> ControlFlow<()> {
        handle_event(event, &sender_clone)
    };
    // Construct the sender and cache it in the thread local variable so we can send messages to it
    let sender = ewebsock::ws_connect(url, options, Box::new(handle_event_closure)).map_err(|e| anyhow::anyhow!("Failed to connect to websocket: {}", e))?;
    Ok((sender, event_receiver))
}

fn handle_event(event: WsEvent, event_sender: &Sender<MapEvent>) -> ControlFlow<()> {
    match handle_event_inner(event, event_sender) {
        Ok(control_flow) => control_flow,
        Err(err) => {
            error!("Error in handle_event: {}", err);
            ControlFlow::<()>::Continue(())
        }
    }
}

fn handle_event_inner(event: WsEvent, event_sender: &Sender<MapEvent>) -> anyhow::Result<ControlFlow<()>> {
    match event {
        Message(message) => {
            match message {
                Binary(data) => {
                    // Decode MapEvent from the binary data
                    let event = rkyv::from_bytes::<MapEvent, rkyv::rancor::Error>(&data)
                        .context("Failed to decode MapEvent from binary data")?;
                    
                    event_sender.send(event)
                        .context("Failed to send frame to channel")?;
                }
                Text(_) => warn!("Received text message, this is not supported"),
                Unknown(_) => warn!("Received unknown message, this is not supported"),
                Ping(_) => warn!("Received ping message, this is not supported"),
                Pong(_) => warn!("Received pong message, this is not supported")
            }
        },
        Error(error) => warn!("Websocket Error: {:?}", error),
        Opened => info!("Opened socket"),
        Closed => {
            event_sender.send(MapEvent::ConnectionClosed).context("Failed to send ConnectionClosed event to channel")?;
        }
    }
    Ok(ControlFlow::<()>::Continue(()))
}