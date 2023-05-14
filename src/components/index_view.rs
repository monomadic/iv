use std::{collections::HashMap, path::PathBuf};

use image::{DynamicImage, GenericImageView};
use pixels::Pixels;

use crate::state::AppState;

use super::Component;

#[derive(Default)]
pub struct IndexView {
    width: u32,
    height: u32,
    thumbcache: HashMap<String, DynamicImage>,
}

impl Component for IndexView {
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn update(&mut self, _state: &mut AppState) -> bool {
        true
    }

    fn draw(
        &mut self,
        state: &crate::state::AppState,
        config: &crate::config::Config,
        pixels: &mut pixels::Pixels,
    ) {
        let thumb_width = self.width / state.cols;

        let thumbs: Vec<DynamicImage> = state
            .assets
            .assets
            .iter()
            .filter_map(|path| {
                let hash = self.hash(&path, thumb_width);

                if let Some(cached_thumb) = self.thumbcache.get(&hash) {
                    Some(cached_thumb.clone())
                } else {
                    let processed_thumb =
                        process_image(path, thumb_width, config.thumbnail_padding)?;
                    self.thumbcache.insert(hash, processed_thumb.clone());
                    Some(processed_thumb)
                }
            })
            .collect();

        // render
        self.render_index_view(
            &thumbs,
            pixels,
            state.cols,
            state.rowskip,
            config.thumbnail_padding,
            state.cursor(),
        );
    }
}

impl IndexView {
    fn hash(&self, path: &PathBuf, width: u32) -> String {
        format!("{:?}#{}", path, width)
    }

    pub fn render_index_view(
        &self,
        thumbs: &Vec<DynamicImage>,
        pixels: &mut Pixels,
        cols: u32,
        rowskip: u32,
        padding: u32,
        selected: usize,
    ) {
        let thumb_width = self.width / cols;
        let thumb_height = thumb_width;
        let pixels_frame = pixels.frame_mut();

        // black bg
        for pixel in pixels_frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }

        // the maximum amount of images displayed on screen
        let images_max = cols as usize * (self.height as f64 / thumb_height as f64).ceil() as usize;

        for (i, thumb) in thumbs
            .iter()
            .skip((rowskip * cols) as usize)
            .take(images_max)
            .enumerate()
        {
            let thumb_aspect_ratio = 1.0; // thumb_width as f32 / thumb_height as f32;
            let (image_width, image_height) = thumb.dimensions();
            let image_aspect_ratio = image_width as f32 / image_height as f32;

            let (new_width, new_height) = if image_aspect_ratio > thumb_aspect_ratio {
                (
                    thumb_width,
                    (thumb_width as f32 / image_aspect_ratio) as u32,
                )
            } else {
                (
                    (thumb_height as f32 * image_aspect_ratio) as u32,
                    thumb_height,
                )
            };

            let x_offset = (i as u32 % cols) * thumb_width + (thumb_width - new_width) / 2;
            let y_offset = (i as u32 / cols) * thumb_height + (thumb_height - new_height) / 2;

            let x_offset = x_offset + padding;
            let y_offset = y_offset + padding;

            for (x, y, pixel) in thumb.pixels() {
                let position = (((y + y_offset) * self.width) + (x + x_offset)) as usize;
                let rgba = pixel.0;

                // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
                if position * 4 + 4 <= pixels_frame.len() {
                    pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
                }
            }

            // Draw border for the selected thumbnail
            if i + (rowskip * cols) as usize == selected {
                let border_color = [255, 255, 255, 255]; // White border
                let border_thickness = 10;

                for y in 0..thumb_height {
                    for x in 0..thumb_width {
                        if x < border_thickness
                            || x >= thumb_width - border_thickness
                            || y < border_thickness
                            || y >= thumb_height - border_thickness
                        {
                            let position = (((y + (i as u32 / cols) * thumb_height) * self.width)
                                + x
                                + (i as u32 % cols) * thumb_width)
                                as usize;

                            // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
                            if position * 4 + 4 <= pixels_frame.len() {
                                pixels_frame[(position * 4)..(position * 4 + 4)]
                                    .copy_from_slice(&border_color);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn process_image(path: &PathBuf, width: u32, padding: u32) -> Option<DynamicImage> {
    let width = width - (padding * 2);
    image::open(path).map(|i| i.thumbnail(width, width)).ok()
}
