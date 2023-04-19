use crate::{collection::AssetCollection, prelude::*};
use std::path::Path;

#[derive(Default)]
pub struct AppState {
    pub assets: AssetCollection,
    pub layout: LayoutState,
    pub cols: u32,
}

#[derive(Default)]
pub enum LayoutState {
    #[default]
    SingleView,
    // Filmstrip,
    IndexView,
}

impl AppState {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        // show gallery view if a directory was passed as an argument
        // otherwise show the single image fullscreen
        let layout = if path.is_dir() {
            LayoutState::IndexView
        } else {
            LayoutState::SingleView
        };
        let assets = AssetCollection::new(path)?;

        Ok(AppState {
            assets,
            layout,
            cols: 6,
        })
    }

    pub fn toggle_layout(&mut self) {
        use LayoutState::*;
        self.layout = match self.layout {
            SingleView => IndexView,
            IndexView => SingleView,
        };
    }

    pub fn cursor(&self) -> usize {
        self.assets.cursor
    }

    pub fn up(&mut self) {
        self.assets.decrement(self.cols as usize);
    }

    pub fn down(&mut self) {
        self.assets.advance(self.cols as usize);
    }

    pub fn left(&mut self) {
        self.assets.prev();
    }

    pub fn right(&mut self) {
        self.assets.next();
    }
}
