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
        let assets = AssetCollection::new(path.into())?;
        let layout = if assets.cursor != 0 {
            LayoutState::MultiView
        } else {
            LayoutState::SingleView
        };

        Ok(AppState { assets, layout })
    }

    pub fn window_loaded() {}
    pub fn draw() -> Vec<u32> {
        todo!()
    }
}
