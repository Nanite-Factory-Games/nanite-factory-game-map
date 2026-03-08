use std::time::Duration;

use bevy::prelude::*;

mod components;
pub mod events;
mod observers;

use lightyear::connection::direction::NetworkDirection;
use lightyear::prelude::*;

use crate::server::protocol::events::ServerConnectMessage;

pub struct ServerMessageChannel;

pub fn build(app: &mut App) {
    // Setup channel for server to client messages
    app
        .add_channel::<ServerMessageChannel>(ChannelSettings {
            mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
            send_frequency: Duration::default(),
            priority: 1.0,
        })
        .add_direction(NetworkDirection::ServerToClient);

    // Register event for connection to server
    app
        .register_event::<ServerConnectMessage>()
        .add_direction(NetworkDirection::ServerToClient);

    // Add observers for new client and connected client
    app
        .add_observer(observers::handle_new_client)
        .add_observer(observers::handle_connected);

    // Add delta compression manager
    let server = app
        .world_mut()
        .query_filtered::<Entity, With<Server>>()
        .single(app.world_mut())
        .expect("Failed to get server entity when constructing world delta manager");

    app.world_mut()
        .entity_mut(server)
        .insert(DeltaManager::default());
}