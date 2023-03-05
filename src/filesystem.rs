use crate::prelude::*;
use std::path::PathBuf;

pub struct FileCollection {
    input_paths: Vec<PathBuf>,
}

impl FileCollection {
    pub fn from_glob(glob: &str) -> Result<Self> {
        let input_paths = glob::glob(glob)?
            .filter_map(|path| path.ok())
            .filter(|path| path.file_name().unwrap() != ".DS_Store")
            .collect();

        Ok(Self { input_paths })
    }

    pub fn next(&mut self) -> Option<PathBuf> {
        // get the next valid image
        while let Some(next_image) = self.input_paths.pop() {
            return Some(next_image);
        }
        None
    }
}
