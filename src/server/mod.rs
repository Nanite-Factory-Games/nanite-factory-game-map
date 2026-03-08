use std::time::Duration;

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use lightyear::{netcode::PRIVATE_KEY_BYTES, prelude::server::ServerPlugins};

use crate::server::protocol::events::ServerConnectMessage;

pub mod entities;
pub mod protocol;

pub const SEND_INTERVAL: Duration = Duration::from_millis(100);

pub fn build(app: &mut App) {
    app
        .add_plugins(entities::build)
        .add_plugins(protocol::build);
}

pub fn configure(tick_duration: Duration, event_rx: Receiver<ServerConnectMessage>, event_tx: Sender<ServerConnectMessage>) -> App {
    let mut app = App::new();

    // Set up lightyear server plugins
    app.add_plugins(ServerPlugins { tick_duration });
    app.insert_resource(Time::<Fixed>::from_duration(tick_duration));
    
    build(&mut app);
    app
}

/// Reads and parses the LIGHTYEAR_PRIVATE_KEY environment variable into a private key.
pub fn parse_private_key_from_env() -> Option<[u8; PRIVATE_KEY_BYTES]> {
    let Ok(key_str) = std::env::var("LIGHTYEAR_PRIVATE_KEY") else {
        return None;
    };
    let private_key: Vec<u8> = key_str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',')
        .collect::<String>()
        .split(',')
        .map(|s| {
            s.parse::<u8>()
                .expect("Failed to parse number in private key")
        })
        .collect();

    if private_key.len() != PRIVATE_KEY_BYTES {
        panic!("Private key must contain exactly {PRIVATE_KEY_BYTES} numbers",);
    }

    let mut bytes = [0u8; PRIVATE_KEY_BYTES];
    bytes.copy_from_slice(&private_key);
    Some(bytes)
}
