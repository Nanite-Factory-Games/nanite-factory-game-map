use bevy::{prelude::*};
use bevy_ecs_tilemap::{tiles::{TilePos, TileStorage}, TilemapBundle};

use crate::shared::events::{TileClickEvent, TileDownEvent, TileUpEvent};


pub fn on_tile_click(
    trigger: Trigger<Pointer<Click>>,
    q_tile_pos: Query<&TilePos>,
    mut ev_click: EventWriter<TileClickEvent>,
) {
    let entity = trigger.entity();
    let tile_pos = *q_tile_pos.get(entity).expect("Failed to get tilepos for tile entity");
    let button = trigger.button;
    ev_click.send(TileClickEvent {
        entity,
        button,
        tile_pos
    });
}

pub fn on_tile_down(
    trigger: Trigger<Pointer<Down>>,
    q_tile_pos: Query<&TilePos>,
    mut ev_click: EventWriter<TileDownEvent>,
) {
    let entity = trigger.entity();
    let tile_pos = *q_tile_pos.get(entity).expect("Failed to get tilepos for tile entity");
    let button = trigger.button;
    let location = trigger.pointer_location.clone();
    ev_click.send(TileDownEvent {
        entity,
        button,
        location,
        tile_pos
    });
}

pub fn on_tile_up(
    trigger: Trigger<Pointer<Up>>,
    q_tile_pos: Query<&TilePos>,
    mut ev_click: EventWriter<TileUpEvent>,
) {
    let entity = trigger.entity();
    let tile_pos = *q_tile_pos.get(entity).expect("Failed to get tilepos for tile entity");
    let button = trigger.button;
    let location = trigger.pointer_location.clone();
    ev_click.send(TileUpEvent {
        entity,
        button,
        location,
        tile_pos
    });
}
