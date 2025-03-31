use bevy::prelude::*;

pub mod events;

pub fn common(app: &mut App) {
    app
        .add_event::<events::TileClickEvent>();
}