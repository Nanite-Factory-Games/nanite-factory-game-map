use bevy::prelude::*;

pub mod events;

pub fn shared(app: &mut App) {
    app
        .add_event::<events::TileClickEvent>();
}