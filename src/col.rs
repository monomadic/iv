use std::path::PathBuf;

use image::DynamicImage;
use multicache::MultiCache;

// https://github.com/woelper/oculante/blob/master/src/scrubber.rs

/// Single threaded cache
pub struct ImageCollection {
    cache: MultiCache<PathBuf, ImageCache>,
    cursor: Cursor,
}

#[derive(Default)]
pub enum ImageCache {
    #[default]
    Uncached,
    Cached(DynamicImage),
}

impl ImageCollection {
    pub fn new() -> Self {
        Self {
            cache: MultiCache::new(500),
            cursor: Cursor::default(),
        }
    }

    pub fn add_path(&mut self, path: PathBuf) {
        self.cache.put(path, ImageCache::Uncached)
    }
}

#[derive(Default)]
pub struct Cursor {
    mode: CursorMode,
    position: usize,
    length: usize,
}

#[derive(Default)]
pub enum CursorMode {
    Linear,
    #[default]
    Cycle,
    Random,
}
