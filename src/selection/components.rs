use bevy::{ecs::{component::Component, system::Resource}, math::Vec2};

#[derive(Component)]
pub struct SelectionBoxMarker;

#[derive(Component)]
pub struct ContextModalMarker;

#[derive(Resource, Default)]
pub struct SelectionBoxDrawing {
    pub start_pos: Option<Vec2>,
    pub current_pos: Vec2,
}