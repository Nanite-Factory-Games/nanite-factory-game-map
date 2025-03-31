use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;

const SELECTABLE_GROUP: Group = Group::GROUP_1;
const SELECTION_GROUP: Group = Group::GROUP_2;


#[derive(Default, Resource)]
pub struct SelectedEntities {
    pub value: HashSet<Entity>,
}

#[derive(Bundle)]
pub struct SelectableBundle {
    collider: Collider,
    transform_bundle: Transform,
    active_collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    sensor: Sensor,
    collision_groups: CollisionGroups,
}

impl SelectableBundle {
    pub fn new(size_x_in_meters: f32, size_y_in_meters: f32, translation: Vec2) -> Self {
        Self {
            collider: Collider::cuboid(
                /* half_x */ size_y_in_meters / 2.0,
                /* half_y */ size_x_in_meters / 2.0,
            ),
            transform_bundle: Transform {
                translation: translation.extend(0.0),
                ..default()
            },
            active_collision_types: ActiveCollisionTypes::STATIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            sensor: Sensor,
            collision_groups: CollisionGroups::new(SELECTABLE_GROUP, SELECTION_GROUP),
        }
    }
}