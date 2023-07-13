use crate::{filesystem::get_images_from_directory, prelude::*};
use image::{io::Reader as ImageReader, DynamicImage};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Default, Clone)]
pub struct AssetCollection {
    pub assets: HashMap<PathBuf, DynamicImage>,
    pub collection: Vec<PathBuf>,
    pub cursor: usize,
}

impl AssetCollection {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let images = get_images_from_directory(&path)?;

        // find the current index
        // unnecessary; just write during the iter
        let index = images.iter().position(|p| p.as_path() == path).unwrap_or(0);

        let mut assets = HashMap::new();
        for image in images {
            if let Ok(image) = ImageReader::open(&path)?.decode() {
                assets.insert(path.to_path_buf(), image);
            }
        }

        let current_selection = assets.keys().map(|path| path.clone()).collect();

        Ok(AssetCollection {
            assets,
            collection: current_selection,
            cursor: index,
        })
    }

    pub fn current(&self) -> Option<&DynamicImage> {
        self.collection
            .get(self.cursor)
            .map(|path| self.assets.get(path))
            .flatten()
    }

    pub fn increment(&mut self, inc: usize) -> bool {
        let next = self.cursor + inc;
        if next < self.assets.len() {
            self.cursor = next;
        } else {
            self.cursor = self.assets.len() - 1;
        }
        true
    }

    pub fn decrement(&mut self, dec: usize) -> bool {
        if self.cursor > dec {
            self.cursor = self.cursor - dec;
        } else {
            self.cursor = 0;
        }
        true
    }

    pub fn thumbs(&self) -> Vec<&DynamicImage> {
        self.collection
            .iter()
            .map(|path| {
                self.assets.get(path)
                // let hash = self.hash(&path, thumb_width);
                // if let Some(cached_thumb) = self.cache.get(&hash) {
                //     Some(cached_thumb.clone())
                // } else {
                //     let processed_thumb =
                //         process_image(path, thumb_width, config.thumbnail_padding)?;
                //     self.cache.insert(hash, processed_thumb.clone());
                //     Some(processed_thumb)
                // }
            })
            .flatten()
            .collect()
    }

    // /// Get the next asset
    // pub fn prev(&mut self) -> Option<&PathBuf> {
    //     if self.cursor == 0 {
    //         self.cursor = self.assets.len() - 1;
    //     } else {
    //         self.cursor -= 1;
    //     }
    //
    //     self.assets.get(self.cursor)
    // }
    //
    // /// Get the next asset
    // pub fn next(&mut self) -> Option<&PathBuf> {
    //     if self.cursor == self.assets.len() - 1 {
    //         self.cursor = 0;
    //     } else {
    //         self.cursor += 1;
    //     }
    //
    //     self.assets.get(self.cursor)
    // }
}
