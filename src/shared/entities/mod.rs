use bevy::prelude::*;
use lightyear::prelude::*;

pub mod components;
pub mod resources;

pub fn build(app: &mut App) {
    app
        // .add_components(components::CharacterBundle)
        // .add_components(components::ResourceEntity)
        .register_component::<components::CharacterPosition>();
        
    app.insert_resource(resources::CharacterIdMap::default());
}