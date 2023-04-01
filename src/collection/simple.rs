use crate::prelude::*;
use image::DynamicImage;
use std::path::PathBuf;

// use crate::cache::simple::SimpleCache;

// TODO: study multicache crate

#[derive(Default)]
pub struct AssetCollection {
    // assets: CycleVec<PathBuf>,
    pub assets: Vec<PathBuf>,
    // cache: SimpleCache<DynamicImage>,
}

// impl Job<DynamicImage> for DynamicImage {
//     fn process(&mut self) -> DynamicImage {
//         DynamicImage::from(
//     }
// }

impl AssetCollection {
    pub fn process(path: PathBuf) -> Result<DynamicImage> {
        image::open(path).map_err(|_| FBIError::Generic("yes".into()))
    }

    /// Get the next asset
    pub fn next(&mut self) -> Result<PathBuf> {
        if let Some(path) = self.assets.pop() {
            return Ok(path);
            //return image::open(path).map_err(|_| FBIError::Generic("yes".into()));
        }

        panic!("bad")
    }
}
