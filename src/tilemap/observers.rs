use bevy::{prelude::*};

use crate::shared::events::{TileClickEvent, TileDownEvent, TileUpEvent};


pub fn on_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut ev_click: EventWriter<TileClickEvent>,
) {
    let entity = trigger.entity();
    let button = trigger.button;
    ev_click.send(TileClickEvent {
        entity,
        button,
    });
}

pub fn on_tile_down(
    trigger: Trigger<Pointer<Down>>,
    mut ev_click: EventWriter<TileDownEvent>,
) {
    let entity = trigger.entity();
    let button = trigger.button;
    let location = trigger.pointer_location.clone();
    dbg!(location.clone());
    ev_click.send(TileDownEvent {
        entity,
        button,
        location,
    });
}

pub fn on_tile_up(
    trigger: Trigger<Pointer<Up>>,
    mut ev_click: EventWriter<TileUpEvent>,
) {
    let entity = trigger.entity();
    let button = trigger.button;
    let location = trigger.pointer_location.clone();
    ev_click.send(TileUpEvent {
        entity,
        button,
        location,
    });
}



// pub fn on_tile_move(
//     trigger: Trigger<Pointer<Move>>,
//     mut ev_click: EventWriter<TileMoveEvent>,
// ) {
//     let entity = trigger.entity();
//     let location = trigger.pointer_location.clone();
//     ev_click.send(TileMoveEvent {
//         entity,
//         location,
//     });
// }