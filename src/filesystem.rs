use crate::prelude::*;
use image::io::Reader as ImageReader;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_images_from_glob(arg: &str) -> Result<Vec<PathBuf>> {
    Ok(glob::glob(&arg)?
        .filter_map(|e| e.ok())
        .filter(|path| is_image(&path))
        .collect())
}

// pub fn get_images_from_directory<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
//     get_target_directory(path.as_ref()).and_then(get_image_paths)
// }

pub fn get_images_from_dir<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    get_target_directory(path.as_ref()).and_then(get_image_keys)
}

pub fn get_files_from_directory<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    match path.as_ref().parent() {
        Some(parent_path) => {
            let entries = WalkDir::new(parent_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() && e.path() != path.as_ref())
                .map(|e| String::from(e.path().to_string_lossy()))
                .collect::<Vec<String>>();

            Ok(entries)
        }
        None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path")),
    }
}

fn get_target_directory(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        Ok(path.to_owned())
    } else if path.is_file() {
        path.parent()
            .map(|p| {
                if p == Path::new("") {
                    std::env::current_dir().unwrap()
                } else {
                    p.to_path_buf()
                }
            })
            .ok_or(IVError::Static("Unable to get the parent directory"))
    } else {
        Err(IVError::Static("Invalid path argument"))
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

fn get_image_keys<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    Ok(fs::read_dir(dir.as_ref())?
        .filter_map(|result| result.ok())
        .map(|entry| entry.path())
        .filter(|path| is_image(&path))
        .filter_map(|p| p.to_str().map(String::from))
        .collect::<Vec<String>>())
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
        let path = Path::new("./assets");
        let files = get_files_from_directory(path).unwrap();

        assert!(!files.is_empty());
        assert_eq!(files.len(), 12);
    }

    #[test]
    fn test_get_valid_image_from_valid_path() {
        let path = Path::new("assets/girl.jpg");
        let result = get_images_from_directory(path);
        assert!(result.is_ok());

        let images = result.unwrap();
        // Test that the images Vec is not empty, and each path points to a valid image
        assert!(!images.is_empty());
        assert_eq!(
            images
                .iter()
                .flat_map(|p| p.to_str())
                .collect::<Vec<&str>>()
                .len(),
            12
        );

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
