use crate::prelude::*;
use std::path::PathBuf;

pub struct FileCollection {
    input_paths: Vec<PathBuf>,
    buffer_cache: Vec<U32Buffer>,
}

pub struct U32Buffer {
    pub buffer: Vec<u32>,
    pub width: u16,
    pub height: u16,
}

impl FileCollection {
    pub fn from_glob(glob: &str) -> Result<Self> {
        let input_paths = glob::glob(glob)?
            .filter_map(|path| path.ok())
            .filter(|path| path.file_name().unwrap() != ".DS_Store")
            .collect();

        println!("{:?}", input_paths);

        Ok(Self {
            input_paths,
            buffer_cache: Vec::new(),
        })
    }

    pub fn next(&mut self) -> Option<&U32Buffer> {
        while let Some(path) = self.input_paths.pop() {
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

            self.buffer_cache.push(u32_buffer);

            return self.buffer_cache.last();
        }

        None
    }

    pub fn current(&self) -> &U32Buffer {
        self.buffer_cache.last().expect("buffer cache is empty")
    }
}
