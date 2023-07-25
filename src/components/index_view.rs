use image::{DynamicImage, GenericImage, Pixel};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use pixels::Pixels;

use crate::{config::Config, msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct IndexView {
    width: u32,
    height: u32,
}

impl Component for IndexView {
    fn update(&mut self, msg: &Msg, state: &mut AppState, config: &Config) -> bool {
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

    fn draw(&mut self, state: &AppState, config: &Config, surface: &mut Pixels) {
        let cols = state.cols as f32;
        let rows = self.height as f32 / (self.width as f32 / cols);
        let padding = config.thumbnail_padding;

        let component_width = self.width as f32;
        let component_height = self.height as f32;

        let thumbnail_width = component_width / cols;
        let thumbnail_height = component_height / rows;

        // let selected = state.cursor() as f32;

        let mut buffer = image::ImageBuffer::new(self.width, self.height);

        // get a framebuffer from the pixels context
        let frame = surface.frame_mut();
        // draw the thumb background
        // black bg
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }

        // the maximum amount of images displayed on screen
        // let images_max = cols as usize * (self.height as f64 / thumb_height as f64).ceil() as usize;

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
                .get(path, width as u32, height as u32)
                .expect("image to be cached");

            // Draw rounded corners for the thumbnail
            // let mut thumb = thumb.clone();
            // let corner_radius = 10; // Adjust the radius as per your preference

            let i = i as f32;

            // calculate x position
            let x_offset = (i % cols) * thumbnail_width;
            // calculate y position
            let y_offset = (i / cols).floor() * thumbnail_height;

            // center horizontally
            let x_offset = x_offset + (thumbnail_width - thumb.width() as f32) / 2.0;
            // center vertically
            let y_offset = y_offset + (thumbnail_height - thumb.height() as f32) / 2.0;

            buffer
                .copy_from(&thumb.to_rgba8(), x_offset as u32, y_offset as u32)
                .unwrap();

            // // Copy thumbnail to the framebuffer
            //
            // for (x, y, pixel) in thumb.pixels() {
            //     // // Draw border for the selected thumbnail
            //     // if i + (rowskip * cols) == selected {
            //     //     let border_color = [255, 255, 255, 255]; // White
            //     //     let cols = cols as u32;
            //     //     let i = i as u32;
            //     //     let component_width = component_width as u32;
            //     //     let thumbnail_width = thumbnail_width as u32;
            //     //     let thumbnail_height = thumbnail_height as u32;
            //     //     let rowskip = self.rowskip(state.collection.cursor, state.cols as f32);
            //     //     let border_thickness = config.thumbnail_border_thickness;
            //     //
            //     //     for y in 0..thumbnail_height {
            //     //         for x in 0..thumbnail_width {
            //     //             if x < border_thickness
            //     //                 || x >= thumb_width - border_thickness
            //     //                 || y < border_thickness
            //     //                 || y >= thumb_height - border_thickness
            //     //             {
            //     //                 let position = (((y + (i / cols) * thumb_height) * component_width)
            //     //                     + x
            //     //                     + (i % cols) * thumb_width)
            //     //                     as usize;
            //     //
            //     //                 // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
            //     //                 if position * 4 + 4 <= pixels_frame.len() {
            //     //                     pixels_frame[(position * 4)..(position * 4 + 4)]
            //     //                         .copy_from_slice(&border_color);
            //     //                 }
            //     //             }
            //     //         }
            //     //     }
            //     // }
            // }
        }
        frame.copy_from_slice(&buffer.into_raw())
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
