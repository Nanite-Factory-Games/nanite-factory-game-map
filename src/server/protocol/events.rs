use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct ServerConnectMessage {
    /// Map of entity ids to character ids
    pub character_id_map: bimap::BiBTreeMap<Entity, u16>,
    /// Tick duration of the server
    pub tick_duration: Duration,
}

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct CharacterIdAddMessage(pub Entity, pub u16);
