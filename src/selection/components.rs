use bevy::{ecs::{component::Component, entity::Entity}, gizmos::gizmos::Gizmos, math::Vec2, utils::HashSet};
use bevy::prelude::*;
use bevy::color::Color;

#[derive(Component)]
pub struct MarqueeSelection {
    pub start: Vec2,
    pub end: Vec2,
}

impl MarqueeSelection {
    pub fn display_gizmos(&self, gizmos: &mut Gizmos) {
        gizmos.circle_2d(self.start, 0.125, Color::srgb(1.0, 0.0, 0.0));
        gizmos.circle_2d(self.end, 0.125, Color::srgb(1.0, 0.0, 0.0));
    }
}