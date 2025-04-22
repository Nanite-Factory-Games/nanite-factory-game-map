use bevy::{ecs::query, prelude::*};
use bevy_aseprite_ultra::prelude::{Animation, AseSpriteAnimation};

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn((
    //     AseSpriteAnimation {
    //         aseprite: asset_server.load("player.aseprite"),
    //         animation: Animation::tag("idle_down"),
    //     },
    //     Transform::from_xyz((1.0 * 16.0), (1.0 * 16.0), 48.0),
    // ));
}

// We want to update animation timings if the tickrate changes so that the animations match
// pub fn on_tickrate_change(
//     mut query: Query<&mut AsepriteAnimation>,
//     time: Res<Time::<Fixed>>,
// ) {
//     if !time.is_changed() || time.is_added() { return};
//     for mut animation in query.iter_mut() {
//         animation.animation_total_duration_ms = Some(time.timestep().as_millis() as u64 * 10);
//     }
// }

// example of how to make player transparent
// pub fn on_player_spawned(mut query: Query<&mut Sprite, With<AsepriteAnimation>>) {
//     for mut sprite in query.iter_mut() {
//         sprite.color = Color::srgba(1.0, 1.0, 1.6, 0.6);
//     }
// }