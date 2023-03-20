use crate::{layout::LayoutState, AssetCollection};

#[derive(Default)]
pub struct App {
    pub collection: AssetCollection,
    layout: LayoutState,
}

impl App {
    pub fn new(collection: AssetCollection) -> Self {
        App {
            collection,
            ..Default::default()
        }
    }
    pub fn window_loaded() {}
    pub fn draw() -> Vec<u32> {
        todo!()
    }
}
