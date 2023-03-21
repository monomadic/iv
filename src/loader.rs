// loading strategy

use std::path::{Path, PathBuf};

use crate::prelude::*;

// TODO: return a cursor position, etc

fn is_supported_image(entry: &PathBuf) -> bool {
    entry
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ["jpg", "jpeg", "png", "bmp", "tiff", "ico"].contains(&ext))
        .unwrap_or(false)
}

pub fn path_from_args(arg: &str) -> Result<Vec<PathBuf>> {
    Ok(glob::glob(&arg)?
        .filter_map(|e| e.ok())
        .filter(is_supported_image)
        .collect())
}

pub fn get_folder(path: PathBuf) -> PathBuf {
    if path.is_file() {
        // return surrounding dir
        path.parent().unwrap_or(&Path::new("/"))
    } else {
        &path.as_path()
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(path_from_args("").unwrap().is_empty())
    }

    #[test]
    fn test_single_file() {
        assert_eq!(path_from_args("assets/cyberpunk.jpg").unwrap().len(), 1);
    }

    #[test]
    fn test_glob() {
        assert_eq!(path_from_args("assets/*.*").unwrap().len(), 4);
    }

    #[test]
    fn test_get_folder() {
        assert_eq!(
            get_folder("assets/cyberpunk.jpg".into()),
            PathBuf::from("assets")
        );
    }
}
