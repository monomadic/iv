use pixels::Pixels;

use crate::{config::Config, msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct IndexView {
    width: u32,
    height: u32,
}

impl Component for IndexView {
    fn update(&mut self, state: &mut AppState, config: &Config, msg: &Msg) -> bool {
        // move the selected thumbnail
        match msg {
            Msg::MoveUp => state.collection.decrement(state.cols as usize),
            Msg::MoveDown => state.collection.increment(state.cols as usize),
            Msg::MoveLeft => state.collection.decrement(1),
            Msg::MoveRight => state.collection.increment(1),
            Msg::Resized(width, height) => {
                self.width = *width;
                self.height = *height;
                // precache
                for key in
                    self.visible_images(&state, state.cols as f32, config.thumbnail_padding as f32)
                {
                    let (width, height) = self
                        .inner_image_dimensions(state.cols as f32, config.thumbnail_padding as f32);
                    state.cache(&key, width as u32, height as u32);
                }
            }
        }

        true
    }

    fn draw(&mut self, state: &AppState, config: &Config, buffer: &mut Pixels) {
        let padding = config.thumbnail_padding;

        let cols = state.cols as f32;
        let rows = self.height as f32 / (self.width as f32 / cols);

        let frame_w = self.width as f32;
        let frame_h = self.height as f32;

        let thumbnail_width = frame_w / cols;
        let thumbnail_height = frame_h / rows;

        let selected = state.cursor();
        let rowskip = self.rowskip(selected, cols);

        buffer.clear_color(pixels::wgpu::Color::BLACK);
        crate::image::clear(buffer);

        // get a framebuffer from the pixels context
        // let frame = surface.frame_mut();
        // let mut rgba = image::Rgba::from_slice(frame);

        // the maximum amount of images displayed on screen
        // let images_max = cols as usize * (self.height as f64 / thumb_height as f64).ceil() as usize;

        // let mut buffer = image::ImageBuffer::new(self.width, self.height);

        // let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        //     ImageBuffer::from_raw(self.width, self.height, frame.into()).unwrap();

        for (i, path) in self
            .visible_images(&state, cols, padding as f32)
            .iter()
            .enumerate()
        {
            // Retrieve thumbnail from the cache
            let (width, height) =
                self.inner_image_dimensions(cols as f32, config.thumbnail_padding as f32);

            let thumb = state
                .cache
                .get(path, width as u32)
                .expect("image to be cached");

            // let thumb = crate::image::inset_border(thumb.clone(), 10.0);

            let i = i as f32;

            // calculate x position
            let offset_x = (i % cols) * thumbnail_width;
            // calculate y position
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
            if i == selected as f32 + (rowskip * cols) {
                crate::image::border(
                    buffer,
                    frame_w,
                    frame_h,
                    offset_x,
                    offset_y,
                    thumb.width() as f32,
                    thumb.height() as f32,
                    10.0,
                );
            }
        }
    }
}

impl IndexView {
    fn inner_image_dimensions(&self, cols: f32, padding: f32) -> (f32, f32) {
        let width = self.thumb_height(cols, padding) - padding * 2.0;
        let height = self.thumb_height(cols, padding) - padding * 2.0;
        (width, height)
    }

    fn visible_images(&self, state: &AppState, cols: f32, padding: f32) -> Vec<String> {
        state
            .collection
            .keys
            .iter()
            .skip((self.rowskip(state.cursor(), cols) * cols) as usize)
            .take(self.number_of_visible_images(cols as f32, padding) as usize)
            .map(|p| p.clone())
            .collect()
    }

    // The maximum amount of images displayed on screen
    fn number_of_visible_images(&self, cols: f32, padding: f32) -> f32 {
        cols * (self.height as f32 / self.thumb_height(cols, padding)).ceil()
    }

    fn thumb_height(&self, cols: f32, padding: f32) -> f32 {
        self.width as f32 / cols - padding * 2.0
    }

    // fn get_thumb(&self, path: &str, cols: u32) -> &DynamicImage {}

    fn rowskip(&self, cursor: usize, cols: f32) -> f32 {
        // let total_rows = (total_assets as f64 / cols as f64).ceil() as u32;
        let col_width = (self.width as f32 / cols).floor();
        let current_row = (cursor as f32 / cols) + 1.0;
        let rows_on_screen = self.height as f32 / col_width;

        if current_row > rows_on_screen {
            current_row - rows_on_screen as f32
        } else {
            0.0
        }
    }
}
