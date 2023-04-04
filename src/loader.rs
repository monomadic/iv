// loading strategy

use walkdir::{DirEntry, WalkDir};

use crate::prelude::*;
use std::path::PathBuf;

// TODO: return a cursor position, etc

pub fn parse_arg(path: PathBuf) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        // todo: cycle with position etc
        return Ok(vec![path]);
    }

    if path.is_dir() {
        return Ok(WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(DirEntry::into_path)
            .filter(is_supported_image)
            .collect());
    }

    if !path.exists() {
        panic!("bad path");
    }

    todo!()
}

fn is_supported_image(entry: &PathBuf) -> bool {
    entry
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ["jpg", "jpeg", "png", "bmp", "tiff", "ico"].contains(&ext))
        .unwrap_or(false)
}

pub fn glob_from_arg(arg: &str) -> Result<Vec<PathBuf>> {
    Ok(glob::glob(&arg)?
        .filter_map(|e| e.ok())
        .filter(is_supported_image)
        .collect())
}

pub fn paths_from_arg(arg: &str) -> Result<Vec<PathBuf>> {
    let mut files = glob_from_arg(arg)?;
    if files.len() == 1 {
        // walkdir?
    }

    Ok(files)
}

pub fn get_surrounding_files(file: PathBuf) -> Result<Vec<PathBuf>> {
    todo!()
}

pub fn get_folder_for_file(path: &PathBuf) -> Option<PathBuf> {
    if !path.exists() {
        return None;
    }
    if path.is_file() {
        // return surrounding dir
        path.parent().map(PathBuf::from)
    } else {
        Some(path.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(glob_from_arg("").unwrap().is_empty())
    }

    #[test]
    fn test_single_file() {
        assert_eq!(glob_from_arg("assets/cyberpunk.jpg").unwrap().len(), 1);
    }

    #[test]
    fn test_glob() {
        assert_eq!(glob_from_arg("assets/*.*").unwrap().len(), 4);
    }

    #[test]
    fn test_get_folder() {
        assert_eq!(
            get_folder_for_file(&PathBuf::from("assets/cyberpunk.jpg")),
            Some(PathBuf::from("assets"))
        );
    }
}
