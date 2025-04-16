use std::{io::{Cursor, ErrorKind}, path::Path, sync::Arc};

use bevy::{asset::{io::Reader, AssetLoader, AssetPath}, log, prelude::*, utils::HashMap};
use bevy_ecs_tilemap::map::TilemapTexture;
use anyhow::Context;


struct BytesResourceReader {
    bytes: Arc<[u8]>,
}

impl BytesResourceReader {
    fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: Arc::from(bytes),
        }
    }
}

impl tiled::ResourceReader for BytesResourceReader {
    type Resource = Cursor<Arc<[u8]>>;
    type Error = std::io::Error;

    fn read_from(&mut self, _path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        // In this case, the path is ignored because the byte data is already provided.
        Ok(Cursor::new(self.bytes.clone()))
    }
}

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,

    pub tilemap_textures: HashMap<usize, TilemapTexture>,
}

pub struct TiledLoader;

impl AssetLoader for TiledLoader {
    type Asset = TiledMap;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut loader = tiled::Loader::with_cache_and_reader(
            tiled::DefaultResourceCache::new(),
            BytesResourceReader::new(&bytes),
        );
        let map = loader.load_tmx_map(load_context.path()).map_err(|e| {
            std::io::Error::new(ErrorKind::Other, format!("Could not load TMX map: {e}"))
        })?;

        let mut tilemap_textures = HashMap::default();

        for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
            let tilemap_texture = match &tileset.image {
                None => {
                    log::info!("Skipping image collection tileset '{}' which is incompatible with atlas feature", tileset.name);
                    continue;
                }
                Some(img) => {
                    // The load context path is the TMX file itself. If the file is at the root of the
                    // assets/ directory structure then the tmx_dir will be empty, which is fine.
                    let tmx_dir = load_context
                        .path()
                        .parent()
                        .context("The asset load context was empty.")?
                        .to_str()
                        .context("The asset load context was empty.")?
                        .to_string();
                    log::info!("tmx_dir: {:?}", tmx_dir);
                    let image_source = img.source.to_str().context("The image source was empty.")?.to_string();
                    let tile_path = format!("memory://{}{}", tmx_dir, image_source);
                    log::info!("tile_path: {:?}", tile_path);
                    let texture: Handle<Image> = load_context.load(tile_path);

                    TilemapTexture::Single(texture.clone())
                }
            };
            tilemap_textures.insert(tileset_index, tilemap_texture);
        }
        

        let asset_map = TiledMap {
            map,
            tilemap_textures,
        };

        log::info!("Loaded map: {}", load_context.path().display());
        Ok(asset_map)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}