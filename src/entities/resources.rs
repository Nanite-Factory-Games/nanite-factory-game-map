use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource)]
pub struct EntityIdMap(pub HashMap<u64, Entity>);