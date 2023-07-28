use image::io::Reader as ImageReader;
use image::DynamicImage;

use crate::{cache::ImageCache, config::Config, prelude::*};
use std::collections::HashMap;

use super::{collection::AssetCollection, LayoutState};

#[derive(Default)]
pub struct AppState {
    pub files: HashMap<String, DynamicImage>,
    pub collection: AssetCollection,
    pub layout_state: LayoutState,
    pub cache: ImageCache,
    /// number of columns in index view
    pub cols: u32,
    pub placeholder: DynamicImage,

    pub thumbnail_padding: u32,
    pub thumbnail_border_thickness: u32,
}

impl AppState {
    pub fn new(
        layout_state: LayoutState,
        collection: AssetCollection,
        config: Config,
    ) -> Result<Self> {
        // read files into memory
        let mut files = HashMap::new();
        for image_path in &collection.keys {
            if let Ok(image) = ImageReader::open(image_path)?.decode() {
                files.insert(image_path.clone(), image);
            }
        }

        let placeholder = image::load_from_memory(include_bytes!("../../assets/placeholder.jpg"))
            .expect("placeholder was invalid");

        Ok(AppState {
            files,
            collection,
            layout_state,
            cache: ImageCache::default(),
            cols: config.index_columns,
            placeholder,
            thumbnail_padding: config.thumbnail_padding,
            thumbnail_border_thickness: config.thumbnail_border_thickness,
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
        self.cache.write(key, &original, width, height)
    }

    pub fn get_original(&self, path: &str) -> &DynamicImage {
        self.files.get(path).unwrap_or(&self.placeholder)
    }

    pub fn current_image(&self) -> &DynamicImage {
        self.get_original(self.collection.current())
    }
}
