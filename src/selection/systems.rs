use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::shared::{events::{TileClickEvent, TileDownEvent, TileUpEvent}, resources::ControlsEnabled};

use super::{components::{SelectionBoxDrawing, SelectionBoxMarker}, events::SelectionEvent};

// We want to skip when shift is pressed
pub fn tile_click_handler(
    mut event: EventReader<TileClickEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tile_query: Query<(Entity, &TilePos)>,
) {
    
}

pub fn tile_down_handler(
    mut commands: Commands,
    mut events: EventReader<TileDownEvent>,
    mut box_drawing: ResMut<SelectionBoxDrawing>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    controls_enabled: Res<ControlsEnabled>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if controls_enabled.0 == false { return; }
    if let Some(event) = events.read().filter(|event| event.button == PointerButton::Primary).last() {
        let window = q_window.single();

        // Get cursor position in world coordinates
        if let Some(cursor_pos) = window.cursor_position() {
            // println!("cursor_pos: {:?}", cursor_pos);
            let (camera, camera_transform) = q_camera.single();
            let pos = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap();
            box_drawing.current_pos = pos;

            box_drawing.start_pos = Some(pos);
            // box_drawing.start_coords = Some(event.location);

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
    }
}

pub fn draw_box_system(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut box_drawing: ResMut<SelectionBoxDrawing>,
    mut query: Query<(&mut Transform, &mut Mesh2d), With<SelectionBoxMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();

    // Get cursor position in world coordinates
    if let Some(cursor_pos) = window.cursor_position() {
        // println!("cursor_pos: {:?}", cursor_pos);
        let (camera, camera_transform) = q_camera.single();
        let pos = camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .unwrap();
        box_drawing.current_pos = pos;


        if let Some(start) = box_drawing.start_pos {
            // Remove existing box
            for (transform, mesh) in query.get_single_mut().iter_mut() {
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

pub fn tile_up_handler(
    mut commands: Commands,
    mut events: EventReader<TileUpEvent>,
    mut box_drawing: ResMut<SelectionBoxDrawing>,
    mut q_selection_box: Query<(Entity, &mut Transform, &mut Mesh2d), With<SelectionBoxMarker>>,
) {
    if let Some(event) = events.read().filter(|event| event.button == PointerButton::Primary).last() {
        if let Some(selection_start) = box_drawing.start_coords {
            commands.send_event(SelectionEvent{
                selection_start,
                selection_end: event.tile_pos,
            });
        }
    
        // Handle mouse button release (stop drawing)
        box_drawing.start_pos = None;
        box_drawing.start_coords = None;
        for (entity, _, _) in q_selection_box.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}
