use bevy::{input::{mouse::MouseButtonInput, ButtonState}, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::shared::events::TileClickEvent;

use super::components::MarqueeSelection;


// We want to skip when shift is pressed
pub fn tile_click_handler(
    mut event: EventReader<TileClickEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tile_query: Query<(Entity, &TilePos)>,
) {
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        return;
    }
    for event in event.read() {
        if let Some(tile) = tile_query.iter().find(|(tile, _)| *tile == event.entity) {
            match event.button {
                PointerButton::Primary => {
                    println!("Primary mouse button clicked");
                    // selected_entities.entity.clear();
                }
                PointerButton::Secondary => {
                    println!("Secondary mouse button clicked");
                },
                PointerButton::Middle => {
                    println!("Middle mouse button clicked");
                },
            }
        }
    }
}

pub fn mouse_input_handler(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_marquee: Query<Entity, With<MarqueeSelection>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.pressed(KeyCode::ShiftLeft) && !keyboard_input.pressed(KeyCode::ShiftRight) {
        return;
    }
    for event in events.read() {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        if let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| Some(camera.viewport_to_world_2d(camera_transform, cursor)))
        {
            match event.state {
                ButtonState::Pressed => {
                    info!("Pressed: {:?} at {:?}", event.button, cursor_position);
                    // let marquee = spawn_cube(&mut commands, cursor_position.extend(0.0), 0.0);
                    // commands.entity(marquee).insert(MarqueeSelection {
                    //     start: cursor_position,
                    //     end: cursor_position,
                    // });
                }
                ButtonState::Released => {
                    info!("Released: {:?} at {:?}", event.button, cursor_position);
                    // if let Ok(marquee) = q_marquee.get_single() {
                    //     commands.entity(marquee).despawn();
                    // }
                }
            }
        }
    }
}