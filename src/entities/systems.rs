use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepriteHandle};

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        AsepriteBundle {
            aseprite: AsepriteHandle(asset_server.load("player.ase")),
            animation: AsepriteAnimation::from("right_walk"),
            ..Default::default()
        },
        
        Transform::from_xyz(0.0, 0.0, 80.0),
    ));
}