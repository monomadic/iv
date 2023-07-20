use image::GenericImageView;

use crate::{config::Config, msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct IndexView {
    width: u32,
    height: u32,
    // cols: f32,
}

impl Component for IndexView {
    fn update(&mut self, msg: Msg, state: &mut AppState, config: &Config) -> bool {
        // move the selected thumbnail
        match msg {
            Msg::MoveUp => state.collection.decrement(state.cols as usize),
            Msg::MoveDown => state.collection.increment(state.cols as usize),
            Msg::MoveLeft => state.collection.decrement(1),
            Msg::MoveRight => state.collection.increment(1),
            Msg::Resized(width, height) => {
                self.width = width;
                self.height = height;
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

        // // TODO: refactor
        // //
        // // update thumbnail cache
        // //
        // let thumbnail_image_width = self.width / state.cols - (config.thumbnail_padding * 2);
        // let rowskip = self.rowskip(state.collection.cursor, state.cols) * state.cols;
        // // the maximum amount of images displayed on screen
        // let len = state.cols as usize
        //     * (self.height as f64 / thumbnail_image_width as f64).ceil() as usize;
        // state.precache_thumbnails(thumbnail_image_width, rowskip as usize, len);

        // state.cache.store(thumbnail_image_width, rowskip as usize, len);

        true
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        let cols = state.cols as f32;
        let rows = self.height as f32 / (self.width as f32 / cols);
        let padding = config.thumbnail_padding;

        let component_width = self.width as f32;
        let component_height = self.height as f32;

        let thumbnail_width = component_width / cols;
        let thumbnail_height = component_height / rows;

        // let selected = state.cursor() as f32;
        // let thumb_width = self.width / state.cols - padding * 2;
        // let thumb_height = thumb_width;

        // draw the thumb background
        let pixels_frame = pixels.frame_mut();
        // black bg
        for pixel in pixels_frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }

        // the maximum amount of images displayed on screen
        // let images_max = cols as usize * (self.height as f64 / thumb_height as f64).ceil() as usize;

        // NEW

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

            let i = i as f32;

            let x_offset = (i % cols) * thumbnail_width;
            let y_offset = (i / cols).floor() * thumbnail_height;

            // Copy thumbnail to the framebuffer
            for (x, y, pixel) in thumb.pixels() {
                let position =
                    (((y as f32 + y_offset) * component_width) + (x as f32 + x_offset)) as usize;
                let rgba = pixel.0;

                // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
                if position * 4 + 4 <= pixels_frame.len() {
                    pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
                }

                // // Draw border for the selected thumbnail
                // if i + (rowskip * cols) == selected {
                //     let border_color = [255, 255, 255, 255]; // White
                //     let cols = cols as u32;
                //     let i = i as u32;
                //     let component_width = component_width as u32;
                //     let thumbnail_width = thumbnail_width as u32;
                //     let thumbnail_height = thumbnail_height as u32;
                //     let rowskip = self.rowskip(state.collection.cursor, state.cols as f32);
                //     let border_thickness = config.thumbnail_border_thickness;
                //
                //     for y in 0..thumbnail_height {
                //         for x in 0..thumbnail_width {
                //             if x < border_thickness
                //                 || x >= thumb_width - border_thickness
                //                 || y < border_thickness
                //                 || y >= thumb_height - border_thickness
                //             {
                //                 let position = (((y + (i / cols) * thumb_height) * component_width)
                //                     + x
                //                     + (i % cols) * thumb_width)
                //                     as usize;
                //
                //                 // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
                //                 if position * 4 + 4 <= pixels_frame.len() {
                //                     pixels_frame[(position * 4)..(position * 4 + 4)]
                //                         .copy_from_slice(&border_color);
                //                 }
                //             }
                //         }
                //     }
                // }
            }
        }

        // END NEW

        // for (i, path) in self
        //     .visible_images(&state, cols, padding as f32)
        //     .iter()
        //     .enumerate()
        // {
        //     // let original = state.get_original(path);
        //     let thumb = state
        //         .cache
        //         .get(path, thumb_width, thumb_width)
        //         .unwrap_or(&state.placeholder);
        //
        //     let thumb_aspect_ratio = 1.0; // thumb_width as f32 / thumb_height as f32;
        //     let (image_width, image_height) = thumb.dimensions();
        //     let image_aspect_ratio = image_width as f32 / image_height as f32;
        //
        //     let (new_width, new_height) = if image_aspect_ratio > thumb_aspect_ratio {
        //         (
        //             thumb_width,
        //             (thumb_width as f32 / image_aspect_ratio) as u32,
        //         )
        //     } else {
        //         (
        //             (thumb_height as f32 * image_aspect_ratio) as u32,
        //             thumb_height,
        //         )
        //     };
        //
        //     let x_offset = (i as u32 % cols) * thumb_width + (thumb_width - new_width) / 2;
        //     let y_offset = (i as u32 / cols) * thumb_height + (thumb_height - new_height) / 2;
        //
        //     // Offset by padding
        //     let x_offset = x_offset + padding;
        //     let y_offset = y_offset + padding;
        //
        //     // Copy thumbnail to the framebuffer
        //     for (x, y, pixel) in thumb.pixels() {
        //         let position = (((y + y_offset) * self.width) + (x + x_offset)) as usize;
        //         let rgba = pixel.0;
        //
        //         // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
        //         if position * 4 + 4 <= pixels_frame.len() {
        //             pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
        //         }
        //     }
        //
        //     // Draw border for the selected thumbnail
        //     if i + (rowskip * cols) as usize == selected {
        //         let border_color = [255, 255, 255, 255]; // White border
        //
        //         for y in 0..thumb_height {
        //             for x in 0..thumb_width {
        //                 if x < border_thickness
        //                     || x >= thumb_width - border_thickness
        //                     || y < border_thickness
        //                     || y >= thumb_height - border_thickness
        //                 {
        //                     let position = (((y + (i as u32 / cols) * thumb_height) * self.width)
        //                         + x
        //                         + (i as u32 % cols) * thumb_width)
        //                         as usize;
        //
        //                     // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
        //                     if position * 4 + 4 <= pixels_frame.len() {
        //                         pixels_frame[(position * 4)..(position * 4 + 4)]
        //                             .copy_from_slice(&border_color);
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
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
