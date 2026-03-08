// This module will handle all entities such as players, enemies, projectiles, etc.

use bevy::prelude::*;

use crate::shared::entities::resources::CharacterIdMap;

// use bevy_aseprite_ultra::AsepriteUltraPlugin;


mod systems;

pub fn build(app: &mut App) {
    app
        // .add_plugins(AsepriteUltraPlugin)
        .insert_resource(CharacterIdMap::default())
        .add_systems(Update, systems::on_character_position_change);
}