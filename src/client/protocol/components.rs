use std::net::{Ipv4Addr, SocketAddr};

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use lightyear::{link::{Link, RecvLinkConditioner}, netcode::{NetcodeClient, NetcodeServer}, prelude::{Authentication, Client, Identity, LocalAddr, PeerAddr, PredictionManager, ReplicationReceiver, client::WebTransportClientIo, server::WebTransportServerIo}};

use crate::{server::parse_private_key_from_env, shared::SharedSettings};

#[derive(Component)]
#[component(on_add = ServerConnectionInfo::on_add)]
pub struct ServerConnectionInfo {
    pub client_id: u32,
    /// The socket address of the server
    pub server_addr: SocketAddr,
    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<RecvLinkConditioner>,
    pub shared: SharedSettings,
}

impl ServerConnectionInfo {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        use lightyear::prelude::client::NetcodeConfig;
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<ServerConnectionInfo>().expect("Failed to find ServerConnectionInfo in world");
            let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0);
            entity_mut.insert((
                Client::default(),
                Link::new(settings.conditioner.clone()),
                LocalAddr(client_addr),
                PeerAddr(settings.server_addr),
                ReplicationReceiver::default(),
                PredictionManager::default(),
                Name::from("Client"),
            ));

            // use dummy zeroed key explicitly here.
            let auth = Authentication::Manual {
                server_addr: settings.server_addr,
                client_id: settings.client_id as u64,
                private_key: settings.shared.private_key,
                protocol_id: settings.shared.protocol_id,
            };
            let netcode_config = NetcodeConfig {
                // Make sure that the server times out clients when their connection is closed
                client_timeout_secs: 3,
                token_expire_secs: -1,
                ..default()
            };
            entity_mut.insert(NetcodeClient::new(auth, netcode_config)?);

            let certificate_digest = "".to_string();
            entity_mut.insert(WebTransportClientIo { certificate_digest });
            Ok(())
        });
    }
}

