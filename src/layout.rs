#[derive(Default)]
pub enum LayoutState {
    #[default]
    SingleView,
    MultiView,
}

impl LayoutState {
    pub fn draw(&self) -> Vec<u32> {
        todo!()
    }
}
