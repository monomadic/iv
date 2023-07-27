use crate::{config::Config, msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct ImageComponent {
    width: u32,
    height: u32,
    // zoom: Zoom,
}

// pub enum Zoom {
//     FitToScreen,
//     Zoom(f32),
// }

impl Component for ImageComponent {
    fn update(&mut self, state: &mut AppState, _config: &Config, msg: &Msg) -> bool {
        match msg {
            Msg::Resized(width, height) => {
                self.width = *width;
                self.height = *height;
            }
            Msg::MoveLeft | Msg::MoveUp => {
                state.collection.decrement(1);
            }
            Msg::MoveRight | Msg::MoveDown => {
                state.collection.increment(1);
            }
        }
        true
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        _config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        pixels.clear_color(pixels::wgpu::Color::BLACK);
        crate::image::clear(pixels);
        crate::image::copy_and_resize(state.current_image(), pixels, self.width, self.height);
    }
}
