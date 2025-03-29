use bevy::prelude::*;

mod systems;

pub fn camera(app: &mut App) {
    app
        .add_systems(Update, systems::movement);
}
