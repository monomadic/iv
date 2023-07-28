use winit::event::VirtualKeyCode;

use crate::{
    components::Component,
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
    fn update(&mut self, state: &mut AppState, msg: &Msg) -> bool {
        match msg {
            Msg::Resized(width, height) => {
                self.width = *width;
                self.height = *height;
                // Resize events should propagate to all components.
                self.solo_view.update(state, msg);
                self.index_view.update(state, msg);
            }
            Msg::KeyPress(key, _modifiers) => match key {
                VirtualKeyCode::Space | VirtualKeyCode::Return => {
                    state.toggle_layout();
                }
                _ => (),
            },
            _ => (),
        }

        // update children
        match state.layout_state {
            LayoutState::SingleView => self.solo_view.update(state, msg),
            LayoutState::IndexView => self.index_view.update(state, msg),
        }
    }

    fn draw(&mut self, state: &crate::state::AppState, pixels: &mut pixels::Pixels) {
        match state.layout_state {
            LayoutState::SingleView => self.solo_view.draw(state, pixels),
            LayoutState::IndexView => self.index_view.draw(state, pixels),
        }
    }
}
