use pixels::Pixels;
use winit::event::VirtualKeyCode;

use crate::{msg::Msg, state::AppState};

use super::{Component, Rect};

#[derive(Default)]
pub struct IndexView;

impl Component for IndexView {
    fn update(&mut self, state: &mut AppState, size: &Rect, msg: &Msg) -> bool {
        // move the selected thumbnail
        match msg {
            Msg::KeyPress(key, modifiers) => match key {
                VirtualKeyCode::G => {
                    if modifiers.shift() {
                        state.collection.move_to_end();
                    } else {
                        state.collection.move_to_beginning();
                    }
                }
                VirtualKeyCode::K | VirtualKeyCode::Up => {
                    state.collection.decrement(state.cols as usize)
                }
                VirtualKeyCode::J | VirtualKeyCode::Down => {
                    state.collection.increment(state.cols as usize)
                }
                VirtualKeyCode::H | VirtualKeyCode::Left => state.collection.decrement(1),
                VirtualKeyCode::L | VirtualKeyCode::Right => state.collection.increment(1),
                _ => (),
            },
            _ => (),
        }

        // precache visible images
        for key in self.visible_images(
            &state,
            size.width,
            size.height,
            state.cols as f32,
            state.thumbnail_padding as f32,
        ) {
            let (width, height) = self.inner_image_dimensions(
                size.width,
                state.cols as f32,
                state.thumbnail_padding as f32,
            );
            state.cache(&key, width as u32, height as u32);
        }
        true
    }

    fn draw(&mut self, state: &AppState, size: &Rect, buffer: &mut Pixels) {
        let padding = state.thumbnail_padding;

        let cols = state.cols as f32;
        let rows = size.height / (size.width / cols);

        let frame_w = size.width;
        let frame_h = size.height;

        let thumbnail_width = frame_w / cols;
        let thumbnail_height = frame_h / rows;

        let selected = state.cursor();
        let rowskip = self.rowskip(size.width, size.height, selected, cols);

        buffer.clear_color(pixels::wgpu::Color::BLACK);
        crate::image::clear(buffer);

        for (i, path) in self
            .visible_images(&state, size.width, size.height, cols, padding as f32)
            .iter()
            .enumerate()
        {
            // Retrieve thumbnail from the cache
            let (width, _height) = self.inner_image_dimensions(
                size.width,
                cols as f32,
                state.thumbnail_padding as f32,
            );

            let thumb = state
                .cache
                .get(path, width as u32)
                .expect("image to be cached");

            let i = i as f32;

            let offset_x = (i % cols) * thumbnail_width;
            let offset_y = (i / cols).floor() * thumbnail_height;

            // center horizontally
            let offset_x = offset_x + (thumbnail_width - thumb.width() as f32) / 2.0;
            // center vertically
            let offset_y = offset_y + (thumbnail_height - thumb.height() as f32) / 2.0;

            // copy current image to buffer
            crate::image::copy_with_offset(
                &thumb,
                buffer,
                frame_w as u32,
                frame_h as u32,
                offset_x as u32,
                offset_y as u32,
            );

            // Draw border for the selected thumbnail
            if i == selected as f32 - (rowskip * cols) {
                crate::image::border(
                    buffer,
                    frame_w,
                    frame_h,
                    offset_x,
                    offset_y,
                    thumb.width() as f32,
                    thumb.height() as f32,
                    state.thumbnail_border_thickness as f32,
                );
            }
        }
    }
}

impl IndexView {
    fn inner_image_dimensions(&self, width: f32, cols: f32, padding: f32) -> (f32, f32) {
        (
            self.thumb_height(width, cols, padding),
            self.thumb_height(width, cols, padding),
        )
    }

    fn visible_images(
        &self,
        state: &AppState,
        width: f32,
        height: f32,
        cols: f32,
        padding: f32,
    ) -> Vec<String> {
        state
            .collection
            .keys
            .iter()
            .skip((self.rowskip(width, height, state.cursor(), cols) * cols) as usize)
            .take(self.number_of_visible_images(width, height, cols as f32, padding) as usize)
            .map(|p| p.clone())
            .collect()
    }

    // The maximum amount of images displayed on screen
    fn number_of_visible_images(&self, width: f32, height: f32, cols: f32, padding: f32) -> f32 {
        cols * (height / self.thumb_height(width, cols, padding)).ceil()
    }

    fn thumb_height(&self, width: f32, cols: f32, padding: f32) -> f32 {
        (width - padding * 2.0 * cols) / cols
    }

    fn rowskip(&self, width: f32, height: f32, cursor: usize, cols: f32) -> f32 {
        let current_row = (cursor as f32 / cols) + 1.0;
        let rows_visible = height / (width / cols);

        if current_row > rows_visible {
            (current_row - rows_visible + 1.0).floor()
        } else {
            0.0
        }
    }
}
