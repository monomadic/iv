// asset loading strategy

use crate::prelude::*;

use image::io::Reader as ImageReader;
use std::fs;
use std::path::{Path, PathBuf};

fn get_target_directory(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        Ok(path.to_owned())
    } else if path.is_file() {
        path.parent()
            .map(Path::to_owned)
            .ok_or(FBIError::Static("Unable to get the parent directory"))
    } else {
        Err(FBIError::Static("Invalid path argument"))
    }
}

fn is_image(path: &Path) -> bool {
    ImageReader::open(path)
        .ok()
        .and_then(|reader| reader.format().map(|format| format.can_read()))
        .unwrap_or(false)
}

fn get_image_paths<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir.as_ref())?
        .filter_map(|result| result.ok())
        .map(|entry| entry.path())
        .filter(|path| is_image(&path))
        .collect::<Vec<PathBuf>>())
}

pub fn get_images_from_directory(path: &Path) -> Result<Vec<PathBuf>> {
    get_target_directory(path).and_then(get_image_paths)
}

pub fn glob_from_arg(arg: &str) -> Result<Vec<PathBuf>> {
    Ok(glob::glob(&arg)?
        .filter_map(|e| e.ok())
        .filter(|path| is_image(&path))
        .collect())
}
