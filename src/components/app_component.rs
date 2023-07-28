use crate::{
    components::Component,
    config::Config,
    msg::Msg,
    state::{AppState, LayoutState},
};

use super::{image_component::ImageComponent, IndexView};

#[derive(Default)]
pub struct AppComponent {
    width: u32,
    height: u32,
    index_view: IndexView,
    solo_view: ImageComponent,
}

impl Component for AppComponent {
    fn update(&mut self, state: &mut AppState, config: &Config, msg: &Msg) -> bool {
        match msg {
            Msg::Resized(width, height) => {
                self.width = *width;
                self.height = *height;
                self.solo_view.update(state, config, msg);
                self.index_view.update(state, config, msg);
            }
            Msg::KeyPress(key) => match key {
                _ => (),
            },
            _ => (),
        }

        // update children
        match state.layout_state {
            LayoutState::SingleView => self.solo_view.update(state, config, msg),
            LayoutState::IndexView => self.index_view.update(state, config, msg),
        }
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        // TODO: render children automatically
        match state.layout_state {
            LayoutState::SingleView => {
                //crate::image::copy_image(state.current_image(), pixels, self.width, self.height);
                self.solo_view.draw(state, config, pixels);
            }
            LayoutState::IndexView => {
                self.index_view.draw(state, config, pixels);
            }
        }
    }
}
