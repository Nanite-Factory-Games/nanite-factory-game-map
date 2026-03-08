use bevy::prelude::*;
use lightyear::connection::client_of::ClientOf;
use lightyear::prelude::*;

use crate::server::SEND_INTERVAL;
use crate::server::protocol::{ServerConnectMessage, ServerMessageChannel};
use crate::shared::entities::resources::CharacterIdMap;

/// When a new client tries to connect to a server, an entity is created for it with the `LinkOf` component.
/// This entity represents the link between the server and that client.
///
/// You can add additional components to update the link. In this case we will add a `ReplicationSender` that
/// will enable us to replicate local entities to that client.
/// 
/// The ReplicationSender and name are cleaned up automatically when the link is removed on disconnect
pub(crate) fn handle_new_client(trigger: On<Add, LinkOf>, mut commands: Commands) {
    commands.entity(trigger.entity).insert((
        EventSender::<ServerConnectMessage>::default(),
        ReplicationSender::new(SEND_INTERVAL, SendUpdatesMode::SinceLastAck, false),
        Name::from("Client"),
    ));
}

/// Send map information and any other pieces of information the client needs
/// on initial connection
pub(crate) fn handle_connected(
    trigger: On<Add, Connected>,
    character_id_map: Res<CharacterIdMap>,
    tick_duration: Res<Time<Fixed>>,
    mut senders: Query<(&RemoteId, &mut EventSender<ServerConnectMessage>), With<ClientOf>>,
) {
    if let Ok((remote_id, mut sender)) = senders.get_mut(trigger.entity) {
        let message = ServerConnectMessage {
            character_id_map: character_id_map.0.clone(),
            tick_duration: tick_duration.timestep(),
        };
        sender.trigger::<ServerMessageChannel>(message);
    }
}