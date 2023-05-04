use crate::{collection::AssetCollection, prelude::*};
use std::path::Path;

#[derive(Default)]
pub struct AppState {
    pub assets: AssetCollection,
    pub layout: LayoutState,
    pub cols: u32,
    pub rowskip: u32,
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
            rowskip: 0,
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

    pub fn current_row(&self) -> u32 {
        (self.assets.cursor as f64 / self.total_rows() as f64) as u32
    }

    pub fn total_rows(&self) -> usize {
        (self.assets.assets.len() as f64 / self.cols as f64).ceil() as usize
    }

    pub fn up(&mut self) {
        self.assets.decrement(self.cols as usize);
    }

    pub fn down(&mut self) {
        self.assets.advance(self.cols as usize);

        // shift entire canvas down
        let rows = (self.cols as f32 / 3.0).floor() as u32; // assume 2:3 aspect
        if self.current_row() > (rows + self.rowskip) {
            self.rowskip = self.current_row() - 2;
        }
    }

    pub fn left(&mut self) {
        self.assets.prev();
    }

    pub fn right(&mut self) {
        self.assets.next();
    }
}
