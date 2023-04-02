use crate::{layout::LayoutState, AssetCollection};

#[derive(Default)]
pub struct AppState {
    pub assets: AssetCollection,
    // cache:
    // cursor:
    layout: LayoutState,
}

impl AppState {
    pub fn new(collection: AssetCollection) -> Self {
        AppState {
            assets: collection,
            ..Default::default()
        }
    }
    pub fn window_loaded() {}
    pub fn draw() -> Vec<u32> {
        todo!()
    }
}
