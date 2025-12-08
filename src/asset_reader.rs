use bevy::asset::io::{
    AssetReader, AssetReaderError, PathStream, Reader,
    memory::{Dir, MemoryAssetReader},
};
use std::path::{Path, PathBuf};

/// Helper function to normalize paths (resolves .. and . components)
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {
                // Skip .
            }
            _ => {
                components.push(component);
            }
        }
    }
    components.iter().collect()
}

/// A wrapper around [`MemoryAssetReader`] that normalizes paths before lookup.
/// This allows relative paths like "map/../sprites/..." to resolve correctly to "sprites/...".
/// 
/// This is useful when assets are referenced with relative paths (e.g., from LDtk maps)
/// that need to be normalized before being looked up in the memory asset store.
#[derive(Clone)]
pub struct NormalizingMemoryAssetReader {
    inner: MemoryAssetReader,
}

impl NormalizingMemoryAssetReader {
    /// Creates a new `NormalizingMemoryAssetReader` wrapping the given `MemoryAssetReader`.
    pub fn new(reader: MemoryAssetReader) -> Self {
        Self { inner: reader }
    }

    /// Creates a new `NormalizingMemoryAssetReader` from a `Dir`.
    pub fn from_dir(dir: Dir) -> Self {
        Self {
            inner: MemoryAssetReader { root: dir },
        }
    }
}

impl AssetReader for NormalizingMemoryAssetReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        // Normalize the path
        let normalized = normalize_path(path);
        let root = self.inner.root.clone();
        
        // Move both into async block
        async move {
            let reader = MemoryAssetReader { root };
            // SAFETY: Both reader and normalized are owned by this async block and live
            // for the entire duration. The Reader returned doesn't actually borrow from
            // the path - it only uses it for lookup. We extend the lifetimes to satisfy
            // the trait requirement.
            let reader_ref: &'a MemoryAssetReader = unsafe {
                std::mem::transmute(&reader)
            };
            let normalized_ref: &'a Path = unsafe {
                std::mem::transmute(normalized.as_path())
            };
            let _keep_alive = (reader, normalized); // Keep both alive
            reader_ref.read(normalized_ref).await
        }.await
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        let normalized = normalize_path(path);
        let root = self.inner.root.clone();
        async move {
            let reader = MemoryAssetReader { root };
            let reader_ref: &'a MemoryAssetReader = unsafe {
                std::mem::transmute(&reader)
            };
            let normalized_ref: &'a Path = unsafe {
                std::mem::transmute(normalized.as_path())
            };
            let _keep_alive = (reader, normalized);
            reader_ref.read_meta(normalized_ref).await
        }.await
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        let normalized = normalize_path(path);
        let root = self.inner.root.clone();
        async move {
            let reader = MemoryAssetReader { root };
            reader.read_directory(&normalized).await
        }.await
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        let normalized = normalize_path(path);
        let root = self.inner.root.clone();
        async move {
            let reader = MemoryAssetReader { root };
            reader.is_directory(&normalized).await
        }.await
    }
}

