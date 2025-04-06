use bevy::{
    color::palettes::css::RED,
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_ecs_tilemap::{helpers::selection, tiles::TilePos};
use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::shared::events::{TileClickEvent, TileDownEvent, TileUpEvent};

use super::components::{ContextModalMarker, SelectionBoxDrawing, SelectionBoxMarker};

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
                }
                PointerButton::Middle => {
                    println!("Middle mouse button clicked");
                }
            }
        }
    }
}

pub fn tile_down_handler(
    mut commands: Commands,
    mut events: EventReader<TileDownEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if !keyboard_input.pressed(KeyCode::ShiftLeft) && !keyboard_input.pressed(KeyCode::ShiftRight) {
        return;
    }
}

pub fn draw_box_system(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut box_drawing: ResMut<SelectionBoxDrawing>,
    mut query: Query<(Entity, &mut Transform, &mut Mesh2d), With<SelectionBoxMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();

    // Get cursor position in world coordinates
    if let Some(cursor_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        // println!("cursor_pos: {:?}", cursor_pos);
        let (camera, camera_transform) = q_camera.single();
        let pos = camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .unwrap();
        box_drawing.current_pos = pos;

        // Handle mouse button press (start drawing)
        if mouse_button.just_pressed(MouseButton::Left) {
            box_drawing.start_pos = Some(pos);

            // Create the rectangle mesh
            let mesh = meshes.add(Rectangle::new(0.0, 0.0));

            // Spawn new box entity
            commands.spawn((
                MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 0.5))),
                Mesh2d(mesh),
                Transform::from_translation(pos.extend(50.0)),
                SelectionBoxMarker,
            ));
        }
        // Handle mouse button release (stop drawing)
        else if mouse_button.just_released(MouseButton::Left) {
            box_drawing.start_pos = None;
            for (entity, _, _) in query.iter_mut() {
                commands.entity(entity).despawn();
            }
        }
        // Draw/update the box while holding the mouse button
        else if mouse_button.pressed(MouseButton::Left) {
            if let Some(start) = box_drawing.start_pos {
                // Remove existing box
                for (entity, transform, mesh) in query.get_single_mut().iter_mut() {
                    // Calculate box properties
                    let min = Vec2::new(start.x.min(pos.x), start.y.min(pos.y));
                    let max = Vec2::new(start.x.max(pos.x), start.y.max(pos.y));
                    let size = max - min;
                    let center = min + size / 2.0;

                    // Create the rectangle mesh
                    let new_mesh = meshes.add(Rectangle::new(size.x, size.y));

                    **transform = Transform::from_translation(center.extend(50.0));
                    **mesh = new_mesh.into();
                }
            }
        }
    }
}

pub fn tile_up_handler(
    mut commands: Commands,
    mut event: EventReader<TileUpEvent>,
    tile_query: Query<(Entity, &TilePos)>,
    q_selection_box: Query<Entity, With<SelectionBoxMarker>>,
) {
    for event in event.read() {
        if let Some(tile) = tile_query.iter().find(|(tile, _)| *tile == event.entity) {
            match event.button {
                PointerButton::Primary => {
                    println!("Primary mouse button released");
                }
                PointerButton::Secondary => {
                    println!("Secondary mouse button released");
                }
                PointerButton::Middle => {
                    println!("Middle mouse button released");
                }
            }
        }
    }
}

pub fn mouse_motion_handler(
    mut commands: Commands,
    mut events: EventReader<CursorMoved>,
    mut q_marquee: Query<(&mut SelectionBoxMarker, &mut Sprite, &mut Transform)>,
) {
}

// Creates a hidden context modal that will be shown when the user right clicks
pub fn setup_context_modal(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
            ContextModalMarker
        )
    )
        .insert(PickingBehavior::IGNORE);
}

// fn spawn_selection_box(
//     commands: &mut Commands,
//     translation: Vec2,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>
// ) -> Entity {
//     commands
//         .spawn((
//             Mesh2d(meshes.add(Rectangle::new(50.0))),
//             MeshMaterial2d(materials.add(ColorMaterial::from_color(RED)))
//         ))
//         .id()
// }
