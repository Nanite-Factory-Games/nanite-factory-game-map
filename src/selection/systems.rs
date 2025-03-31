use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::common::events::TileClickEvent;


pub fn tile_click_handler(
    mut event: EventReader<TileClickEvent>,
    tile_query: Query<(Entity, &TilePos)>,
    // mut selected_entities: ResMut<SelectedEntities>,
) {
    for event in event.read() {
        if let Some(tile) = tile_query.iter().find(|(tile, _)| *tile == event.entity) {
            match event.button {
                PointerButton::Primary => {
                    println!("Primary mouse button clicked");
                    // selected_entities.entity.clear();
                }
                PointerButton::Secondary => {
                    println!("Secondary mouse button clicked");
                },
                PointerButton::Middle => {
                    println!("Middle mouse button clicked");
                },
            }
        }
    }
}