use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::client::common::events::{TileClickEvent, TileDownEvent, TileUpEvent};


pub fn on_tile_click(
    click: On<Pointer<Click>>,
    q_tile_pos: Query<&TilePos>,
    mut ev_click: MessageWriter<TileClickEvent>,
) -> Result<(), BevyError> {
    let entity = click.entity;
    let tile_pos = *q_tile_pos.get(entity)?;
    let button = click.button;
    ev_click.write(TileClickEvent {
        entity,
        button,
        tile_pos
    });
    Ok(())
}

pub fn on_tile_down(
    press: On<Pointer<Press>>,
    q_tile_pos: Query<&TilePos>,
    mut ev_click: MessageWriter<TileDownEvent>,
) -> Result<(), BevyError> {
    let entity = press.entity;
    let tile_pos = *q_tile_pos.get(entity)?;
    let button = press.button;
    let location = press.pointer_location.clone();
    ev_click.write(TileDownEvent {
        entity,
        button,
        location,
        tile_pos
    });
    Ok(())
}

pub fn on_tile_up(
    release: On<Pointer<Release>>,
    q_tile_pos: Query<&TilePos>,
    mut ev_click: MessageWriter<TileUpEvent>,
) -> Result<(), BevyError> {
    let entity = release.entity;
    let tile_pos = *q_tile_pos.get(entity)?;
    let button = release.button;
    let location = release.pointer_location.clone();
    ev_click.write(TileUpEvent {
        entity,
        button,
        location,
        tile_pos
    });
    Ok(())
}
