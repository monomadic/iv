use std::{collections::HashMap, path::PathBuf};

use image::{imageops::FilterType, DynamicImage, GenericImageView};
use pixels::Pixels;

use crate::app::{AppState, LayoutState};

pub struct RenderCache {
    width: u32,
    height: u32,
    thumbcache: HashMap<PathBuf, DynamicImage>,
}

fn process_image(path: &PathBuf, width: u32) -> Option<DynamicImage> {
    image::open(path).map(|i| i.thumbnail(width, width)).ok()
}

impl RenderCache {
    pub fn init(width: u32, height: u32) -> Self {
        // start thumbnail processing on bg thread
        Self {
            width,
            height,
            thumbcache: HashMap::new(),
        }
    }

    pub fn draw(&mut self, state: &AppState, pixels: &mut Pixels) {
        match state.layout {
            LayoutState::SingleView => {
                let path = state.assets.current().expect("no current");
                // let image = self
                //     .image_cache
                //     .get(path)
                //     .expect("image not found in cache");
                let image = image::open(path).expect("image open");
                self.render_single_view_op(&image, pixels);
            }
            LayoutState::IndexView => {
                // TODO: cache
                let thumb_width = self.width / state.cols;

                let thumbs: Vec<DynamicImage> = state
                    .assets
                    .assets
                    .iter()
                    .filter_map(|path| {
                        if let Some(cached_thumb) = self.thumbcache.get(path) {
                            Some(cached_thumb.clone())
                        } else {
                            let processed_thumb = process_image(path, thumb_width)?;
                            self.thumbcache
                                .insert(path.clone(), processed_thumb.clone());
                            Some(processed_thumb)
                        }
                    })
                    .collect();

                // let thumbs: Vec<DynamicImage> = state
                //     .assets
                //     .assets
                //     .iter()
                //     .flat_map(image::open)
                //     .map(|i| i.thumbnail(thumb_width, thumb_width))
                //     .collect();

                // render
                self.render_index_view(
                    &thumbs,
                    pixels,
                    state.cols,
                    state.rowskip,
                    5,
                    state.cursor(),
                );
            }
        };
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
        // let thumb_width = thumbs.get(0).map(|t| t.width()).unwrap_or(0);
        // let thumb_height = thumbs.get(0).map(|t| t.height()).unwrap_or(0);
        // let rows = (thumbs.len() as u32 + cols - 1) / cols;

        let thumb_width = self.width / cols;
        let thumb_height = thumb_width;

        let pixels_frame = pixels.frame_mut();

        // black image
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

            let resized_thumb = thumb.resize(
                new_width - (padding * 2),
                new_height - (padding * 2),
                FilterType::Lanczos3,
            );

            for (x, y, pixel) in resized_thumb.pixels() {
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

    // pub fn render_single_view(&self, image: &DynamicImage, pixels: &mut Pixels) {
    //     let resized_image = image.resize(self.width, self.height, FilterType::Lanczos3);
    //     let (resized_width, resized_height) = resized_image.dimensions();
    //     let x_offset = (self.width - resized_width) / 2;
    //     let y_offset = (self.height - resized_height) / 2;
    //
    //     let pixels_frame = pixels.frame_mut();
    //
    //     // black image
    //     for pixel in pixels_frame.chunks_exact_mut(4) {
    //         pixel.copy_from_slice(&[0, 0, 0, 255]);
    //     }
    //
    //     for (x, y, pixel) in resized_image.pixels() {
    //         let position = (((y + y_offset) * self.width) + (x + x_offset)) as usize;
    //         let rgba = pixel.0;
    //         // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
    //         pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
    //     }
    // }

    pub fn render_single_view_op(&self, image: &DynamicImage, pixels: &mut Pixels) {
        let (image_width, image_height) = image.dimensions();
        let image_aspect_ratio = image_width as f32 / image_height as f32;
        let frame_aspect_ratio = self.width as f32 / self.height as f32;

        let (new_width, new_height) = if image_aspect_ratio > frame_aspect_ratio {
            (self.width, (self.width as f32 / image_aspect_ratio) as u32)
        } else {
            (
                (self.height as f32 * image_aspect_ratio) as u32,
                self.height,
            )
        };

        let x_offset = (self.width - new_width) / 2;
        let y_offset = (self.height - new_height) / 2;

        let pixels_frame = pixels.frame_mut();

        // black image
        for pixel in pixels_frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }

        for y in 0..self.height {
            for x in 0..self.width {
                // pixel out of bounds
                if x < x_offset
                    || x >= self.width - x_offset
                    || y < y_offset
                    || y >= self.height - y_offset
                {
                    continue;
                }

                let source_x =
                    ((x - x_offset) as f32 * (image_width as f32 / new_width as f32)) as u32;
                let source_y =
                    ((y - y_offset) as f32 * (image_height as f32 / new_height as f32)) as u32;

                // Clamp the source_x and source_y values within valid bounds
                let clamped_source_x = source_x.min(image_width - 1);
                let clamped_source_y = source_y.min(image_height - 1);

                let pixel = image.get_pixel(clamped_source_x, clamped_source_y);
                let rgba = pixel.0;

                let position = ((y * self.width) + x) as usize;
                // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
                pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
            }
        }
    }
}
