use image::{imageops::FilterType, DynamicImage, RgbaImage};
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
        let layout = match state.layout {
            LayoutState::SingleView => {
                let path = state.assets.current().unwrap();
                // let image = self
                //     .image_cache
                //     .get(path)
                //     .expect("image not found in cache");
                let image = image::open(path).unwrap();
                let image = image.resize(self.width, self.height, FilterType::Lanczos3);
                self.render_single_view(&image.to_rgba8(), pixels);
            }
            LayoutState::MultiView => {
                // let thumbs = state
                //     .assets
                //     .assets
                //     .iter()
                //     .map(|path| image::open(&path))
                //     .collect();

                todo!();
            }
        };
    }

    pub fn render_single_view(&self, image: &RgbaImage, pixels: &mut Pixels) {
        let frame = pixels.frame_mut();

        // align horizontal center by calculating left offset
        // let left_offset = self.width / 2 - image.width() / 2;

        frame.copy_from_slice(&image);
    }
}
