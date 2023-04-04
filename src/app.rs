use crate::prelude::*;
use std::path::PathBuf;

use crate::{layout::LayoutState, AssetCollection};

#[derive(Default)]
pub struct AppState {
    pub assets: AssetCollection,
    pub layout: LayoutState,
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

        Ok(AppState { assets, layout })
    }

    pub fn toggle_layout(&mut self) {
        use LayoutState::*;
        self.layout = match self.layout {
            SingleView => MultiView,
            MultiView => SingleView,
        };
    }

    pub fn preload() {}
    pub fn draw() -> Vec<u32> {
        todo!()
    }
}
