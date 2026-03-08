use std::net::{Ipv4Addr, SocketAddr};

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use lightyear::{link::RecvLinkConditioner, netcode::NetcodeServer, prelude::{Identity, LocalAddr, server::{NetcodeConfig, WebTransportServerIo}}};

use crate::{server::parse_private_key_from_env, shared::SharedSettings};


#[derive(Component, Debug)]
#[component(on_add = ServerConfiguration::on_add)]
pub struct ServerConfiguration {
    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<RecvLinkConditioner>,
    /// Port the server will listen on
    pub local_port: u16,
    /// Certificate settings for the server
    pub certificate: Identity,
    pub shared: SharedSettings,
}

impl ServerConfiguration {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        use lightyear::prelude::server::NetcodeConfig;
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<ServerConfiguration>().unwrap();
            entity_mut.insert((Name::from("Server"),));

            let add_netcode = |entity_mut: &mut EntityWorldMut| {
                // Use private key from environment variable, if set. Otherwise from settings file.
                let private_key = if let Some(key) = parse_private_key_from_env() {
                    info!("Using private key from LIGHTYEAR_PRIVATE_KEY env var");
                    key
                } else {
                    settings.shared.private_key
                };
                entity_mut.insert(NetcodeServer::new(NetcodeConfig {
                    protocol_id: settings.shared.protocol_id,
                    private_key,
                    ..Default::default()
                }));
            };
            
            add_netcode(&mut entity_mut);
            let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), settings.local_port);
            entity_mut.insert((
                LocalAddr(server_addr),
                WebTransportServerIo {
                    certificate: settings.certificate,
                },
            ));
            Ok(())
        });
    }
}