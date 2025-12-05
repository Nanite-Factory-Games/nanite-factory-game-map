use bevy::prelude::*;
use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::Commands, input::mouse::MouseButton};
use bevy_pancam::{DirectionKeys, PanCam};

use crate::MapConfiguration;

pub fn setup(mut commands: Commands, config: Res<MapConfiguration>) {
    commands.spawn((
        Camera2d,
        PanCam {
            // which buttons should drag the camera
            grab_buttons: vec![MouseButton::Middle], 
            // the keyboard buttons used to move the camera
            move_keys: DirectionKeys {
                up:    vec![],
                down:  vec![],
                left:  vec![],
                right: vec![],
            },
            // the speed for the keyboard movement
            speed: 400.,
            // when false, controls are disabled. See toggle example.
            enabled: false,
            // whether to zoom towards the mouse or the center of the screen
            zoom_to_cursor: true,
            // prevent the camera from zooming too far in
            min_scale: 0.1,
            // prevent the camera from zooming too far out
            max_scale: 40.,
            // minimum x position of the camera window
            min_x: f32::NEG_INFINITY,
            // maximum x position of the camera window
            max_x: f32::INFINITY,
            // minimum y position of the camera window
            min_y: f32::NEG_INFINITY,
            // maximum y position of the camera window
            max_y: f32::INFINITY,
        },
        Msaa::Off,
        Transform::from_translation(Vec3::new(config.camera_position.x * 16., config.camera_position.y * 16., 0.)),
    ));
}

/// Updates camera position when the resource is updated
pub fn on_configuration_change(
    mut query: Query<(&mut Transform, &mut PanCam)>,
    config: Res<MapConfiguration>,
) {
    if !config.is_changed() { return; }
    for (mut transform, mut pancam) in query.iter_mut() {
        // Update camera position
        *transform = Transform::from_translation(Vec3::new(config.camera_position.x * 16., config.camera_position.y * 16., 0.));
        
        // Update controls enabled
        pancam.enabled = config.controls_enabled;
    }
}