use crate::prelude::*;
use image::DynamicImage;
use std::{collections::HashMap, path::PathBuf};

// multicache? https://docs.rs/multicache/latest/multicache/
#[derive(Default)]
pub struct FileCache {
    // objects: HashMap<String, CachedImage>,
    objects: HashMap<String, CachedImage>,
}

pub enum CachedImage {
    Unloaded(PathBuf),
    Loaded(DynamicImage),
}

impl CachedImage {}

pub struct U32Buffer {
    pub buffer: Vec<u32>,
    // pub thumbnail: Vec<u32>,
    pub width: u16,
    pub height: u16,
}

// https://github.com/woelper/oculante/blob/master/src/scrubber.rs
#[derive(Default)]
pub struct ImageCollection {
    cache: FileCache,
    cursor: FileCursor,
}

impl ImageCollection {
    pub fn add_image(&mut self, path: PathBuf) {}

    pub fn add_images(glob: &str) -> Result<Self> {
        let mut assets = ImageCollection::default();

        let input_paths: Vec<PathBuf> = glob::glob(glob)?
            .filter_map(|path| path.ok())
            .filter(|path| path.file_name().unwrap() != ".DS_Store")
            .collect();

        for path in input_paths {
            assets.add_image(path);
        }

        Ok(assets)
    }
}

#[derive(Default)]
pub struct FileCursor {
    mode: CursorMode,
    position: usize,
}

#[derive(Default)]
pub enum CursorMode {
    Linear,
    #[default]
    Rotate,
    Random,
}

impl FileCache {
    pub fn next(&mut self) -> Option<&U32Buffer> {
        while let Some(path) = self.objects.pop() {
            let image = image::open(path).expect("image load failed");
            let width = image.width() as u16;
            let height = image.height() as u16;

            // convert from &[u8] to Vec<u32>
            let buffer: Vec<u32> = image
                .as_bytes()
                .chunks(3)
                .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
                .collect();

            let u32_buffer = U32Buffer {
                buffer,
                width,
                height,
            };

            self.image_buffers.push(u32_buffer);

            return self.image_buffers.last();
        }

        None
    }

    pub fn current(&self) -> &U32Buffer {
        self.image_buffers.last().expect("buffer cache is empty")
    }
}
