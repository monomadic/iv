use std::{collections::HashMap, path::PathBuf};

use image::DynamicImage;
use softbuffer::GraphicsContext;

use crate::{app::AppState, layout::LayoutState};

pub struct SoftBufferRenderer {
    buffer: Vec<u32>,
    ctx: GraphicsContext,
    image_cache: HashMap<PathBuf, DynamicImage>,
    thumb_cache: HashMap<PathBuf, DynamicImage>,
    width: u16,
    height: u16,
}

impl SoftBufferRenderer {
    pub fn new(width: u16, height: u16, mut ctx: GraphicsContext) -> Self {
        let buffer = vec![0x111111; width as usize * height as usize];

        // black screen
        ctx.set_buffer(&buffer, width, height);

        SoftBufferRenderer {
            // create screen buffer (black screen)
            buffer,
            ctx,
            image_cache: HashMap::new(),
            thumb_cache: HashMap::new(),
            width,
            height,
        }
    }

    // occurs in the main thread
    pub fn draw(&mut self, state: &AppState) {
        // check for newly processed images

        // TODO: remove this alloc
        let view = DynamicImage::new_rgb8(self.width as u32, self.height as u32);

        let layout = match state.layout {
            LayoutState::SingleView => {
                let path = state.assets.current().unwrap();
                // let image = self
                //     .image_cache
                //     .get(path)
                //     .expect("image not found in cache");
                let image = image::open(path).unwrap();
                crate::layout::render_single_view(&image, view)
                    .expect("failed to render single view")
            }
            LayoutState::MultiView => {
                todo!();
            }
        };

        self.ctx.set_buffer(
            &crate::layout::image_to_u32(layout),
            self.width,
            self.height,
        );
    }
}
