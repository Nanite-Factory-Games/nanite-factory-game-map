use bevy::prelude::*;

use crate::shared::entities::components::CharacterPosition;

/// Update the transform of the entity client side based on the changed character position
/// TODO: Animate this client side
pub fn on_character_position_change(
    mut query: Query<(&CharacterPosition, &mut Transform), Changed<CharacterPosition>>,
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(16., 16., 49.0) * Vec3::new(position.0 as f32, position.1 as f32, 1.0);
    }
}