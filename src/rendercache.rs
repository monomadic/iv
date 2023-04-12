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
        let _layout = match state.layout {
            LayoutState::SingleView => {
                let path = state.assets.current().unwrap();
                // let image = self
                //     .image_cache
                //     .get(path)
                //     .expect("image not found in cache");
                let image = image::open(path).unwrap();
                self.render_single_view(&image, pixels);
            }
            LayoutState::MultiView => {
                let thumbs: Vec<DynamicImage> =
                    state.assets.assets.iter().flat_map(image::open).collect();

                todo!();
            }
        };
    }

    pub fn render_single_view(&self, image: &DynamicImage, pixels: &mut Pixels) {
        let resized_image = image.resize(self.width, self.height, FilterType::Lanczos3);
        let (resized_width, resized_height) = resized_image.dimensions();
        let x_offset = (self.width - resized_width) / 2;
        let y_offset = (self.height - resized_height) / 2;

        let pixels_frame = pixels.frame_mut();

        for (x, y, pixel) in resized_image.pixels() {
            let position = (((y + y_offset) * self.width) + (x + x_offset)) as usize;
            let rgba = pixel.0;

            // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
            pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
        }
    }
}
