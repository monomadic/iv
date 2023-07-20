use crate::msg::Msg;

use super::Component;

#[derive(Default)]
pub struct ImageComponent {
    width: u32,
    height: u32,
    key: String,
}

impl Component for ImageComponent {
    fn update(
        &mut self,
        msg: crate::msg::Msg,
        _state: &mut crate::state::AppState,
        _config: &crate::config::Config,
    ) -> bool {
        match msg {
            Msg::Resized(width, height) => {
                self.width = width;
                self.height = height;
                true
            }
            _ => false,
        }
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        todo!()
    }
}
