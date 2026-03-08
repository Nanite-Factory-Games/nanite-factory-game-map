use std::collections::HashMap;
use std::path::Path;
use bevy::math::Vec2;
use cfg_if;
use serde::{Serialize, Deserialize};

#[cfg(feature = "client")]
pub mod client;
// pub mod timeline;
#[cfg(feature = "client")]
pub mod wasm_api;
#[cfg(feature = "server")]
pub mod server;

pub mod shared;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub url: String,
    pub token: Option<String>
}

#[derive(bevy::ecs::resource::Resource, Clone, Serialize, Deserialize, Debug)]
pub struct ServerConfiguration {
    pub tickrate: u64,
}