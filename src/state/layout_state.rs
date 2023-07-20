use std::path::Path;

#[derive(Default)]
pub enum LayoutState {
    #[default]
    SingleView,
    // Filmstrip,
    IndexView,
}

impl From<&Path> for LayoutState {
    fn from(path: &Path) -> Self {
        // show gallery view if a directory was passed as an argument
        // otherwise show the single image fullscreen
        if path.is_dir() {
            LayoutState::IndexView
        } else {
            LayoutState::SingleView
        }
    }
}

impl LayoutState {
    pub fn toggle(&self) -> Self {
        use LayoutState::*;
        match self {
            SingleView => IndexView,
            IndexView => SingleView,
        }
    }
}
