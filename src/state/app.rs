use image::io::Reader as ImageReader;
use image::DynamicImage;

use crate::{cache::ImageCache, filesystem::get_images_from_directory, prelude::*};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::{collection::AssetCollection, LayoutState};

#[derive(Default)]
pub struct AppState {
    pub files: HashMap<PathBuf, DynamicImage>,
    pub collection: AssetCollection,
    pub layout_state: LayoutState,
    pub cache: ImageCache,

    /// number of columns in index view
    pub cols: u32,
    pub placeholder: DynamicImage,
}

impl AppState {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        // load and validate images into memory
        let image_paths = get_images_from_directory(&path)?;

        let mut files = HashMap::new();
        for image_path in &image_paths {
            if let Ok(image) = ImageReader::open(image_path)?.decode() {
                files.insert(image_path.clone(), image);
            }
        }

        let layout_state = LayoutState::from(path);
        let mut collection = AssetCollection::new(
            image_paths
                .iter()
                .flat_map(|p| p.to_str())
                .map(String::from)
                .collect(),
        );
        if path.is_file() {
            collection.set_current(path.to_str().unwrap());
        }

        let placeholder = image::load_from_memory(include_bytes!("../../assets/placeholder.jpg"))
            .expect("placeholder was invalid");

        Ok(AppState {
            files,
            collection,
            layout_state,
            cache: ImageCache::default(), // TODO: new()
            cols: 6,
            placeholder,
        })
    }

    pub fn toggle_layout(&mut self) {
        self.layout_state = self.layout_state.toggle();
    }

    pub fn cursor(&self) -> usize {
        self.collection.cursor
    }

    pub fn cache(&mut self, key: &str, width: u32, height: u32) {
        let original = self.get_original(&key).clone(); // FIXME: do not clone here
        self.cache.store(key, &original, width, height)
    }

    pub fn precache_thumbnails(&mut self, width: u32, skip: usize, len: usize) {
        for path in self.collection.keys.iter().skip(skip).take(len) {
            let image = self.get_original(path);
            self.cache.store(path, &image.clone(), width, width);
        }
    }

    // pub fn paginated_thumbnails(&self, width: u32, skip: usize, len: usize) -> Vec<&DynamicImage> {
    //     self.collection
    //         .keys
    //         .iter()
    //         .skip(skip)
    //         .take(len)
    //         .map(|path| self.thumbnail(path, width))
    //         .collect()
    // }

    pub fn get_original(&self, path: &str) -> &DynamicImage {
        self.files
            .get(&PathBuf::from(path))
            .unwrap_or(&self.placeholder)
    }

    pub fn current_image(&self) -> &DynamicImage {
        self.get_original(self.collection.current())
    }

    /// Get a precached thumbnail or return the placeholder image.
    pub fn thumbnail(&self, path: &PathBuf, width: u32) -> &DynamicImage {
        self.cache
            .get(path.to_str().unwrap(), width, width)
            .unwrap_or(&self.placeholder)
    }
}
