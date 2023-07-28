use pixels::Pixels;
use winit::event::VirtualKeyCode;

use crate::{
    components::Component,
    msg::Msg,
    state::{AppState, LayoutState},
};

use super::{image_component::ImageComponent, IndexView, Rect};

#[derive(Default)]
pub struct AppComponent {
    single_view: ImageComponent,
    index_view: IndexView,
}

impl Component for AppComponent {
    fn update(&mut self, state: &mut AppState, size: &Rect, msg: &Msg) -> bool {
        match msg {
            Msg::Init => {
                self.single_view.update(state, size, msg);
                self.index_view.update(state, size, msg);
            }
            Msg::KeyPress(key, _modifiers) => match key {
                VirtualKeyCode::Space | VirtualKeyCode::Return => {
                    state.toggle_layout();
                }
                _ => (),
            },
        }

        // update children
        match state.layout_state {
            LayoutState::SingleView => self.single_view.update(state, size, msg),
            LayoutState::IndexView => self.index_view.update(state, size, msg),
        }
    }

    fn draw(&mut self, state: &AppState, size: &Rect, pixels: &mut Pixels) {
        match state.layout_state {
            LayoutState::SingleView => self.single_view.draw(state, size, pixels),
            LayoutState::IndexView => self.index_view.draw(state, size, pixels),
        }
    }
}
