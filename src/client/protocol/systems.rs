use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        ServerConnectionInfo {
            client_id: 0,
            server_addr: SERVER_ADDR,
            conditioner: None,
            shared: SharedSettings::default(),
        },
    ));
}