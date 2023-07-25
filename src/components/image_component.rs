use crate::{config::Config, msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct ImageComponent {
    width: u32,
    height: u32,
    key: String,
}

impl Component for ImageComponent {
    fn update(&mut self, msg: &Msg, state: &mut AppState, _config: &Config) -> bool {
        match msg {
            Msg::Resized(width, height) => {
                self.width = *width;
                self.height = *height;
                true
            }
            Msg::MoveLeft | Msg::MoveUp => {
                state.collection.decrement(1);
                true
            }
            Msg::MoveRight | Msg::MoveDown => {
                state.collection.increment(1);
                true
            }
            _ => false,
        }
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        _config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        crate::image::copy_image(state.current_image(), pixels, self.width, self.height);
    }
}
