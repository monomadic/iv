use crate::{loader::get_images_from_directory, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Default, Clone)]
pub struct AssetCollection {
    pub assets: Vec<PathBuf>,
    pub cursor: usize,
}

impl AssetCollection {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(AssetCollection {
            assets: get_images_from_directory(&path.as_ref())?,
            cursor: 0,
        })
    }

    pub fn current(&self) -> Option<&PathBuf> {
        self.assets.get(self.cursor)
    }

    pub fn set(&mut self, new: usize) {
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

    /// Advance by a set number of positions
    /// TODO: remove
    pub fn advance(&mut self, increment: usize) {
        if self.assets.len() < increment {
            return;
        }
        // for _ in 0..increment {
        //     self.next();
        // }
        self.cursor += increment;
        if self.cursor > self.assets.len() {
            self.cursor = self.assets.len()
        }
    }
}
