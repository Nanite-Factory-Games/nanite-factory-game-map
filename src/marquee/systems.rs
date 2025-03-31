use bevy::{input::{mouse::MouseButtonInput, ButtonState}, math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;
use super::{components::MarqueeSelection, resources::SelectableBundle, ENTITY_SIZE_IN_METERS};

pub fn setup(mut commands: Commands) {
    commands.spawn(SelectableBundle::new(
            ENTITY_SIZE_IN_METERS,
            ENTITY_SIZE_IN_METERS,
            vec2(0.45, 0.1),
        )
    );
    commands.spawn(SelectableBundle::new(
        ENTITY_SIZE_IN_METERS,
        ENTITY_SIZE_IN_METERS,
        vec2(-0.45, -0.1),
        )
    );
    
}

pub fn handle_collision_events(mut events: EventReader<CollisionEvent>) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                info!("Collision started between {:?} and {:?}", a, b);
            }
            CollisionEvent::Stopped(a, b, _) => {
                info!("Collision stopped between {:?} and {:?}", a, b);
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
) {
    use bevy::input::ButtonState::*;
    for event in events.read() {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        if let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| Some(camera.viewport_to_world_2d(camera_transform, cursor)))
        {
            match event.state {
                Pressed => {
                    let marquee = spawn_cube(&mut commands, cursor_position.expect("Failed to convert viewport").extend(50.0));
                    commands.entity(marquee).insert(MarqueeSelection {
                        start: cursor_position.expect("Failed to resolve cursor position"),
                        end: cursor_position.expect("Failed to resolve cursor position"),
                    });
                }
                Released => {
                    if let Ok(marquee) = q_marquee.get_single() {
                        commands.entity(marquee).despawn();
                    }
                }
            }
        }
    }
}

fn spawn_cube(commands: &mut Commands, translation: Vec3) -> Entity {
    commands
        .spawn(Collider::cuboid(
            /* half_x */ 10.0 / 2.0,
            /* half_y */ 10.0 / 2.0,
        ))
        .insert(TransformBundle::from(Transform {
            translation,
            ..default()
        }))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor)
        .id()
}

// Done
pub fn draw_marquee_selection(mut gizmos: Gizmos, q_marquee: Query<&MarqueeSelection>) {
    if let Ok(marquee) = q_marquee.get_single() {
        marquee.display_gizmos(&mut gizmos);
    }
}

// Done
pub fn mouse_motion_handler(
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut events: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_marquee: Query<(Entity, &mut MarqueeSelection)>,
) {
    if let Ok((entity, mut marquee)) = q_marquee.get_single_mut() {
        marquee.display_gizmos(&mut gizmos);
        for event in events.read() {
            let (camera, camera_transform) = q_camera.single();
            if let Ok(cursor_position) =
                camera.viewport_to_world_2d(camera_transform, event.position)
            {
                marquee.end = cursor_position;

                let half_extents = (marquee.start - marquee.end).abs() / 2.0;
                let midpoint = (marquee.start + marquee.end) / 2.0;

                commands
                    .entity(entity)
                    .try_insert(Collider::cuboid(half_extents.x, half_extents.y))
                    .try_insert(Transform::from_xyz(midpoint.x, midpoint.y, 0.0));
            }
        }
    }
}