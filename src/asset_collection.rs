use crate::{loader::get_images_from_directory, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Default, Clone)]
pub struct AssetCollection {
    pub assets: Vec<PathBuf>,
    pub cursor: usize,
}

impl AssetCollection {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let images = get_images_from_directory(&path.as_ref())?;
        let index = images
            .iter()
            .position(|p| p.as_path() == path.as_ref())
            .unwrap_or(0);

        Ok(AssetCollection {
            assets: images,
            cursor: index,
        })
    }

    pub fn current(&self) -> Option<&PathBuf> {
        self.assets.get(self.cursor)
    }

    pub fn jump(&mut self, new: usize) {
        if new <= self.assets.len() {
            self.cursor = new;
        } else {
            panic!("index overflow");
        }
    }

    /// Get the next asset
    pub fn prev(&mut self) -> Option<&PathBuf> {
        if self.cursor == 0 {
            self.cursor = self.assets.len() - 1;
        } else {
            self.cursor -= 1;
        }

        self.assets.get(self.cursor)
    }

    /// Get the next asset
    pub fn next(&mut self) -> Option<&PathBuf> {
        if self.cursor == self.assets.len() - 1 {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }

        self.assets.get(self.cursor)
    }
}
