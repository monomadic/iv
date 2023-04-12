use image::{imageops::FilterType, DynamicImage, GenericImageView, RgbaImage};
use pixels::Pixels;

use crate::{app::AppState, layout::LayoutState};

pub struct RenderCache {
    // cache
    width: u32,
    height: u32,
}

impl RenderCache {
    pub fn init(width: u32, height: u32) -> Self {
        // start thumbnail processing on bg thread
        Self { width, height }
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
            LayoutState::MultiView => {
                // let thumbs: Vec<DynamicImage> =
                //     state.assets.assets.iter().flat_map(image::open).collect();
            }
        };
    }

    pub fn render_single_view(&self, image: &DynamicImage, pixels: &mut Pixels) {
        let resized_image = image.resize(self.width, self.height, FilterType::Lanczos3);
        let (resized_width, resized_height) = resized_image.dimensions();
        let x_offset = (self.width - resized_width) / 2;
        let y_offset = (self.height - resized_height) / 2;

        let pixels_frame = pixels.frame_mut();

        // black image
        for pixel in pixels_frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }

        for (x, y, pixel) in resized_image.pixels() {
            let position = (((y + y_offset) * self.width) + (x + x_offset)) as usize;
            let rgba = pixel.0;
            // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
            pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
        }
    }

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
