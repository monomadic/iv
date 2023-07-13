use crate::prelude::*;
use image::io::Reader as ImageReader;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_images_from_glob(arg: &str) -> Result<Vec<PathBuf>> {
    Ok(glob::glob(&arg)?
        .filter_map(|e| e.ok())
        .filter(|path| is_image(&path))
        .collect())
}

pub fn get_images_from_directory<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    get_target_directory(path.as_ref()).and_then(get_image_paths)
}

fn get_target_directory(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        Ok(path.to_owned())
    } else if path.is_file() {
        path.parent()
            .map(Path::to_owned)
            .ok_or(IVError::Static("Unable to get the parent directory"))
    } else {
        Err(IVError::Static("Invalid path argument"))
    }
}

// TODO: unnecessary function, validate later
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_images_from_nonexistent_path() {
        let path = Path::new("/path/does/not/exist");
        let result = get_images_from_directory(path);
        assert!(result.is_err())
    }

    #[test]
    fn test_get_valid_images_from_valid_path() {
        let path = Path::new("assets");
        let result = get_images_from_directory(path);
        assert!(result.is_ok());

        let images = result.unwrap();
        // Test that the images Vec is not empty, and each path points to a valid image
        assert!(!images.is_empty());
        assert_eq!(images.len(), 4);
        for image in images {
            assert!(is_image(&image));
        }
    }

    #[test]
    fn test_is_image_for_valid_image() {
        let path = Path::new("assets/girl.jpg");
        let result = is_image(path);
        assert!(result);
    }

    #[test]
    fn test_is_image_for_invalid_image() {
        let path = Path::new("assets/bad_image.jpg");
        let result = is_image(path);
        assert!(result);
    }
}
