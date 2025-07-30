use anyhow::Context;
use bevy::{log, platform::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use ldtk_rust::bevy::LdtkMap;


use super::components::*;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let handle = LdtkMapHandle(asset_server.load("memory://map/map.ldtk"));

    commands.spawn(LdtkMapBundle {
        ldtk_map: handle,
        ..Default::default()
    });
}

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<LdtkMap>>,
    maps: Res<Assets<LdtkMap>>,
    mut query: Query<(Entity, &LdtkMapHandle, &LdtkMapConfig)>,
    new_maps: Query<&LdtkMapHandle, Added<LdtkMapHandle>>,
) -> Result<(), BevyError>{
    let mut changed_maps = Vec::<AssetId<LdtkMap>>::default();
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                log::info!("Map added!");
                changed_maps.push(*id);
            }
            AssetEvent::Modified { id } => {
                log::info!("Map changed!");
                changed_maps.push(*id);
            }
            AssetEvent::Removed { id } => {
                log::info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == id);
            }
            _ => continue,
        }
    }

    // If we have new map entities, add them to the changed_maps list
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.0.id());
    }

    for changed_map in changed_maps.iter() {
        for (entity, map_handle, map_config) in query.iter_mut() {
            // only deal with currently changed map
            if map_handle.0.id() != *changed_map {
                continue;
            }
            
            if let Some(ldtk_map) = maps.get(&map_handle.0) {
                // Despawn all existing tilemaps for this LdtkMap
                commands.entity(entity).despawn_related::<Children>();

                // Pull out tilesets and their definitions into a new hashmap
                let mut tilesets = HashMap::new();
                ldtk_map.project.defs.tilesets.iter().for_each(|tileset| {
                    tilesets.insert(
                        tileset.uid,
                        (
                            ldtk_map.tilesets.get(&tileset.uid).unwrap().clone(),
                            tileset,
                        ),
                    );
                });

                let default_grid_size = ldtk_map.project.default_grid_size;
                let level = &ldtk_map.project.levels[map_config.selected_level];

                let map_tile_count_x = (level.px_wid / default_grid_size) as u32;
                let map_tile_count_y = (level.px_hei / default_grid_size) as u32;

                let size = TilemapSize {
                    x: map_tile_count_x,
                    y: map_tile_count_y,
                };

                if level.layer_instances.as_ref().is_none() {
                    println!("No layers found in level!");
                    continue;
                } else {
                    println!("Found {} layers in level!", level.layer_instances.as_ref().unwrap().len());
                }

                // We will create a tilemap for each layer in the following loop
                for (layer_id, layer) in level
                    .layer_instances
                    .as_ref()
                    .unwrap()
                    .iter()
                    .rev()
                    .enumerate()
                {
                    if let Some(uid) = layer.tileset_def_uid {
                        let (texture, tileset) = tilesets.get(&uid).unwrap().clone();

                        // Tileset-specific tilemap settings
                        let tile_size = TilemapTileSize {
                            x: tileset.tile_grid_size as f32,
                            y: tileset.tile_grid_size as f32,
                        };

                        // Pre-emptively create a map entity for tile creation
                        let map_entity = commands.spawn_empty().id();

                        // Create tiles for this layer from LDtk's grid_tiles and auto_layer_tiles
                        let mut storage = TileStorage::empty(size);

                        for tile in layer.grid_tiles.iter().chain(layer.auto_layer_tiles.iter()) {
                            let mut position = TilePos {
                                x: (tile.px[0] / default_grid_size) as u32,
                                y: (tile.px[1] / default_grid_size) as u32,
                            };
                            let flip_x = tile.f & 1 != 0;
                            let flip_y = tile.f & 2 != 0;
                            position.y = map_tile_count_y - position.y - 1;
                            let tile_entity = commands
                                .spawn(TileBundle {
                                    position,
                                    tilemap_id: TilemapId(map_entity),
                                    texture_index: TileTextureIndex(tile.t as u32),
                                    flip: TileFlip { x: flip_x, y: flip_y, d: false },
                                    ..default()
                                })
                                .id();

                            storage.set(&position, tile_entity);
                        }

                        

                        let grid_size = tile_size.into();
                        let map_type = TilemapType::default();

                        // Create the tilemap
                        commands.entity(map_entity).insert(TilemapBundle {
                            grid_size,
                            map_type,
                            size,
                            storage,
                            texture: TilemapTexture::Single(texture),
                            tile_size,
                            anchor: TilemapAnchor::Center,
                            transform: Transform::from_xyz(0.0, 0.0, layer_id as f32),
                            ..default()
                        });
                    } else {
                        println!("Loading layer {}", layer.identifier);

                        for entity in layer.entity_instances.iter() {
                            
                            let (image, _) = tilesets
                                .get(
                                    &entity.tile
                                        .as_ref()
                                        .context("An entity was missing tile definition")?
                                        .tileset_uid as _
                                    )
                                .context("An entity was missing tile definition")?
                                .clone();

                            let level_width_pixels = level.px_wid;
                            let level_height_pixels = level.px_hei;
                            

                            
                            
                            let entity_tile = entity.tile.as_ref().context("An entity was missing tile definition")?;
                            let texture_x1 = entity_tile.x as f32;
                            let texture_y1 = entity_tile.y as f32;
                            let texture_x2 = texture_x1 + entity_tile.w as f32;
                            let texture_y2 = texture_y1 + entity_tile.h as f32;

                            let pivot_x = entity.pivot[0] as f32 * entity_tile.w as f32;
                            let pivot_y = entity.pivot[1] as f32 * entity_tile.h as f32;

                            // Adjust to match Bevy's center-based coordinate system and flipped Y
                            let x = entity.px[0] as f32 + ((entity_tile.w - default_grid_size) as f32 / 2.0) - pivot_x - (level_width_pixels as f32 / 2.0) + default_grid_size as f32 / 2.0;
                            let y = (level_height_pixels as f32 - entity.px[1] as f32 - ((entity_tile.h - default_grid_size) as f32 / 2.0) + pivot_y) - (level_height_pixels as f32 / 2.0) - default_grid_size as f32 / 2.0;
                            

                            commands.spawn((
                                Sprite {
                                    image,
                                    rect: Some(Rect::new(texture_x1, texture_y1, texture_x2, texture_y2)),
                                    ..Default::default()
                                },
                                Transform::from_xyz(x, y, layer_id as f32)
                            ));

                            
                            
                            println!("Entity added");
                        }
                    }
                }
            }
        }
        println!("Finished processing maps");
    }
    Ok(())
}