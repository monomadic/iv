use crate::prelude::*;
use std::path::PathBuf;

use crate::{layout::LayoutState, AssetCollection};

#[derive(Default)]
pub struct AppState {
    pub assets: AssetCollection,
    pub layout: LayoutState,
    pub rows: u32,
}

impl AppState {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();
        // show gallery view if a directory was passed as an argument
        // otherwise show the single image fullscreen
        let layout = if path.is_dir() {
            LayoutState::MultiView
        } else {
            LayoutState::SingleView
        };
        let assets = AssetCollection::new(path)?;

        Ok(AppState {
            assets,
            layout,
            rows: 6,
        })
    }

    pub fn toggle_layout(&mut self) {
        use LayoutState::*;
        self.layout = match self.layout {
            SingleView => MultiView,
            MultiView => SingleView,
        };
    }

    pub fn up(&mut self) {
        self.assets.prev();
    }
    pub fn down(&mut self) {
        self.assets.next();
    }
    pub fn left(&mut self) {
        self.assets.prev();
    }
    pub fn right(&mut self) {
        self.assets.next();
    }

    pub fn preload() {}
    pub fn draw() -> Vec<u32> {
        todo!()
    }
}
