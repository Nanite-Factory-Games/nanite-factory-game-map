use std::time;

use bevy::prelude::*;
use bevy::log::info;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
use bevy_tweening::{lens::TransformPositionLens, Animator, Tween};
use crate::{entities::{components::{CharacterEntity, PlayerCharacterMarker}, resources::EntityIdMap}, FrameReceiver, LoopTimeline };

use super::{resources::{FrameType, LoopTimelineIndex, TimelineFrame}, Timeline};

// We switch between movement and animation frames on every tick
pub fn alternate_frame(
    mut frame_type: ResMut<FrameType>,
) {
    *frame_type = match *frame_type {
        FrameType::Movement => FrameType::Action,
        FrameType::Action => FrameType::Movement
    }
}

// Grabs the most recent frame and sets it as the current frame
pub fn advance_timeline(
    mut timeline: ResMut<Timeline>,
    mut current_frame: ResMut<TimelineFrame>,
    loop_timeline: Res<LoopTimeline>,
    mut loop_timeline_index: ResMut<LoopTimelineIndex>
) {
    if loop_timeline.0 {
        loop_timeline_index.0 += 1;
        if loop_timeline_index.0 >= timeline.0.len() {
            loop_timeline_index.0 = 0;
        }
        if let Some(frame) = timeline.0.get(loop_timeline_index.0) {
            *current_frame = frame.clone();
        }
    } else {
        if let Some(frame) = timeline.0.pop_front() {
            *current_frame = frame;
        }
    }
}

/// Grabs all frames from the timeline frame sender and adds them to the timeline
pub fn consume_timeline(
    mut timeline: ResMut<Timeline>,
    timeline_receiver: ResMut<FrameReceiver>,
) {
    timeline.0.extend(timeline_receiver.0.try_iter());
}

pub fn move_characters(
    mut commands: Commands,
    current_frame: Res<TimelineFrame>,
    mut character_entity_map: ResMut<EntityIdMap>,
    asset_server: Res<AssetServer>,
    frame_type: Res<FrameType>,
    character_query: Query<&Transform>,
    tick_rate: Res<Time::<Fixed>>
) {
    if *frame_type != FrameType::Movement { return } 
    // A few things we'll need to do.
    // first we'll need to find all the new entities that need to be spawned
    // Then we'll need to move all the entities that already exist

    let character_map = &mut character_entity_map.0;

    current_frame.character_movements.iter().for_each(|(id, position)| {
        if let Some(entity) = character_map.get(id) {
            let transform = character_query.get(*entity).unwrap();
            let mut end = position.extend(1.0) * Vec3::new(16., 16., 49.0);
            end.x += 8.;
            end.y += 8.;
            if transform.translation.x != end.x || transform.translation.y != end.y {
                // Tween the movement of the character between its current position and the next
                let tween = Tween::new(
                    // Use a quadratic easing on both endpoints.
                    EaseFunction::QuadraticInOut,
                    // Animation time.
                    tick_rate.timestep(),
                    // The lens gives access to the Transform component of the Entity,
                    // for the Animator to animate it. It also contains the start and
                    // end values respectively associated with the progress ratios 0. and 1.
                    TransformPositionLens {
                        start: transform.translation,
                        end,
                    },
                );
                let animation_tag = if transform.translation.y < end.y {
                    "walk_up"
                } else  if transform.translation.y > end.y {
                    "walk_down"
                } else if transform.translation.x < end.x {
                    "walk_right"
                } else {
                    "walk_left"
                };
                let animation = AseAnimation {
                    aseprite: asset_server.load("player.aseprite"),
                    animation: Animation::tag(animation_tag),
                };
                commands
                    .entity(*entity)
                    .insert(Animator::new(tween))
                    .insert(animation);
            }
        } else {
            info!("spawning character {} at {}", id, position);
            let mut tf = Transform::from_translation(position.extend(1.0) * Vec3::new(16., 16., 49.0));
            tf.translation.x += 8.;
            tf.translation.y += 8.;
            let entity = commands.spawn((
                tf,
                CharacterEntity {
                    name: format!("character_{}", id)
                },
                Sprite::default(),
                AseAnimation {
                    aseprite: asset_server.load("player.aseprite"),
                    animation: Animation::tag("idle_down"),
                },
            )).id();
            character_map.insert(*id, entity);
        }
    });
}

pub fn animate_characters(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut character_entity_map: ResMut<EntityIdMap>,
    asset_server: Res<AssetServer>,
) {
    if *frame_type != FrameType::Action { return } 

    let character_map = &mut character_entity_map.0;

    current_frame.character_actions.iter().for_each(|(id, animation_name)| {
        if let Some(entity) = character_map.get(id) {
            let animation = AseAnimation {
                aseprite: asset_server.load("player.aseprite"),
                animation: Animation::tag(&animation_name),
            };
            commands
                .entity(*entity)
                .insert(animation);
        }
    });
}

pub fn move_npcs(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut query: Query<(&CharacterEntity, &Transform), With<PlayerCharacterMarker>>,
) {
    if *frame_type != FrameType::Movement { return } 


}

pub fn animate_npcs(
    mut commands: Commands,
    mut current_frame: Res<TimelineFrame>,
    frame_type: Res<FrameType>,
    mut query: Query<(&CharacterEntity, &Transform), With<PlayerCharacterMarker>>,
) {
    if *frame_type != FrameType::Action { return } 

}
