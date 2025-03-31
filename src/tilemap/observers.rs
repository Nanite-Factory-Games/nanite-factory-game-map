use bevy::prelude::*;

use crate::common::events::TileClickEvent;


pub fn on_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut ev_click: EventWriter<TileClickEvent>,
) {
    info!("Clicked");
    let entity = trigger.entity();
    let button = trigger.button;
    ev_click.send(TileClickEvent {
        entity,
        button,
    });
}