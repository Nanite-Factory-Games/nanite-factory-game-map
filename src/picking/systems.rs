use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Event)]
pub struct TileClickEvent {
    pub entity: Entity,
    pub button: PointerButton,
}

#[derive(Resource)]
pub struct SendHolder {
    pub sender: Sender<Option<String>>,
}

pub fn tile_click_handler(
    mut event: EventReader<TileClickEvent>,
    tile_query: Query<(Entity, &TilePos)>,
    send_holder: Res<SendHolder>,
    mut selected_entities: ResMut<SelectedEntities>,
) {
    for event in event.read() {
        if let Some(tile) = tile_query.iter().find(|(tile, _)| *tile == event.entity) {
            match event.button {
                PointerButton::Primary => {
                    selected_entities.entity.clear();
                }
                PointerButton::Secondary => {
                    if !selected_entities.entity.is_empty() {
                        send_holder
                            .sender
                            .send(Some(
                                serde_json::to_string(&BevyMessage::Move(BevyMoveCommand {
                                    entities: selected_entities
                                        .entity
                                        .iter()
                                        .map(|(_, n)| n.id)
                                        .collect(),
                                    target: websocket::Coord {
                                        x: tile.1.x as u16,
                                        y: tile.1.y as u16,
                                    },
                                }))
                                .unwrap(),
                            ))
                            .unwrap();
                    }
                }
                PointerButton::Middle => todo!(),
            }
        }
    }
}