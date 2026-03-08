use bevy::prelude::*;

pub mod events;

pub fn build(app: &mut App) {
    app
        .add_message::<events::TileClickEvent>()
        .add_message::<events::TileDownEvent>()
        .add_message::<events::TileUpEvent>();
}