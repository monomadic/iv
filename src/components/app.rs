use crate::{components::Component, msg::Msg, state::AppState};

use super::IndexView;

#[derive(Default)]
pub struct App {
    width: u32,
    height: u32,
    index_view: IndexView,
}

impl Component for App {
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        // grid component is full size of window
        self.index_view.resize(width, height);
    }

    fn update(&mut self, msg: Msg, state: &mut AppState) -> bool {
        match state.layout {
            crate::state::LayoutState::SingleView => true,
            crate::state::LayoutState::IndexView => self.index_view.update(msg, state),
        }
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        match state.layout {
            crate::state::LayoutState::SingleView => {
                // let path = state.assets.current().expect("no current");
                // let image = image::open(path).expect("image open");
                // crate::image::copy_image(&image, pixels, self.width, self.height);
            }
            crate::state::LayoutState::IndexView => {
                self.index_view.draw(state, config, pixels);
            }
        }
    }
}
