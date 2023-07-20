use std::path::PathBuf;

use image::GenericImageView;

use crate::{msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct GalleryComponent {
    width: u32,
    height: u32,
}

impl GalleryComponent {
    fn column_width(&self, cols: f32) -> f32 {
        (self.width as f32 / cols).floor()
    }

    fn visible_thumbnails(&self) -> Vec<PathBuf> {
        Vec::new()
    }
}

impl Component for GalleryComponent {
    fn update(&mut self, msg: Msg, state: &mut AppState, config: &crate::config::Config) -> bool {
        // move the selected thumbnail
        match msg {
            Msg::MoveUp => state.collection.decrement(state.cols as usize),
            Msg::MoveDown => state.collection.increment(state.cols as usize),
            Msg::MoveLeft => state.collection.decrement(1),
            Msg::MoveRight => state.collection.increment(1),
            Msg::Resized(width, height) => {
                self.width = width;
                self.height = height;
                // heat up the thumb cache
            }
        }

        true
    }

    fn draw(
        &mut self,
        state: &AppState,
        config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        todo!()
    }
}
