use bevy::prelude::*;

pub mod events;
pub mod resources;

pub fn shared(app: &mut App) {
    app
        .add_event::<events::TileClickEvent>()
        .add_event::<events::TileDownEvent>()
        .add_event::<events::TileUpEvent>();
}