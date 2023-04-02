use crate::prelude::*;
use image::DynamicImage;
use std::{collections::HashMap, path::PathBuf};

#[derive(Default)]
pub struct AssetCollection {
    // assets: CycleVec<PathBuf>,
    pub assets: Vec<PathBuf>,
    cache: HashMap<PathBuf, DynamicImage>,
}

// impl Job<DynamicImage> for DynamicImage {
//     fn process(&mut self) -> DynamicImage {
//         DynamicImage::from(
//     }
// }

impl AssetCollection {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            assets: paths,
            cache: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: PathBuf) -> Result<DynamicImage> {
        image::open(&path).map_err(|_| FBIError::Generic("yes".into()))
    }

    // pub fn get(&mut self, path: PathBuf) -> Result<&DynamicImage> {
    //     if let Some(image) = self.cache.get(&path) {
    //         Ok(image)
    //     } else {
    //         let image = image::open(&path).map_err(|_| FBIError::Generic("yes".into()))?;
    //         self.cache.insert(path, image);
    //         Ok(&image)
    //     }
    // }

    /// Get the next asset
    pub fn next(&mut self) -> Result<PathBuf> {
        if let Some(path) = self.assets.pop() {
            return Ok(path);
            //return image::open(path).map_err(|_| FBIError::Generic("yes".into()));
        }

        panic!("bad")
    }
}
