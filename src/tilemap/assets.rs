use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_tilemap::map::TilemapTexture;

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,

    pub tilemap_textures: HashMap<usize, TilemapTexture>,

    // The offset into the tileset_images for each tile id within each tileset.
    #[cfg(not(feature = "atlas"))]
    pub tile_image_offsets: HashMap<(usize, tiled::TileId), u32>,
}