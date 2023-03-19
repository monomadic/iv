use crate::prelude::*;
use image::DynamicImage;
use std::path::PathBuf;

use crate::cache::simple::SimpleCache;

#[derive(Default)]
pub struct AssetCollection {
    // assets: CycleVec<PathBuf>,
    assets: Vec<PathBuf>,
    cache: SimpleCache<DynamicImage>,
}

// impl Job<DynamicImage> for DynamicImage {
//     fn process(&mut self) -> DynamicImage {
//         DynamicImage::from(
//     }
// }

impl AssetCollection {
    /// Create a new asset collection from PathBufs
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            assets: paths,
            cache: SimpleCache::default(),
        }
    }

    /// Get the next asset
    pub fn next(&mut self) -> Result<DynamicImage> {
        if let Some(path) = self.assets.pop() {
            return image::open(path).map_err(|_| FBIError::Generic("yes".into()));
        }

        panic!("bad")
    }
}
