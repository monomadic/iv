use std::collections::HashMap;

use image::DynamicImage;

/// ImageCache is a caching system for images.
/// It uses a HashMap to store DynamicImage objects, which are keyed by a hash string.
/// The hash string is a combination of a provided key and the desired image dimensions.#[derive(Default)]
#[derive(Default)]
pub(crate) struct ImageCache(HashMap<String, DynamicImage>);

impl ImageCache {
    /// Stores an image in the cache and returns a reference to the cached image.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that is used as part of the hash key for caching.
    /// * `image` - A DynamicImage that is to be cached.
    /// * `width` - The desired width of the cached image.
    /// * `height` - The desired height of the cached image.
    ///
    /// # Description
    ///
    /// If the desired width and height are different than the original image,
    /// the image is resized before being stored in the cache.
    /// If the hash key already exists in the cache, the function will return the already cached image.
    ///
    /// # Returns
    ///
    /// A reference to the DynamicImage that is now stored in the cache.
    pub fn store(
        &mut self,
        key: &str,
        image: DynamicImage,
        width: u32,
        height: u32,
    ) -> &DynamicImage {
        let key: String = self.hash(key, width, height);

        if !self.0.contains_key(&key) {
            let image = image.resize(width, height, image::imageops::FilterType::Nearest);
            self.0.insert(key.clone(), image);
        }

        self.0.get(&key).unwrap()
    }

    /// Creates the hash used as the key for each cache entry.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that is used as part of the hash key for caching.
    /// * `width` - The desired width of the cached image.
    /// * `height` - The desired height of the cached image.
    ///
    /// # Returns
    ///
    /// A String that is used as the hash key for a cache entry.
    fn hash(&self, key: &str, width: u32, height: u32) -> String {
        format!("{}{}{}", key, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

    #[cfg(test)]
    mod tests {
        use super::*;
        use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

        #[test]
        fn test_store() {
            // Initialize ImageCache
            let mut cache = ImageCache::default();

            // Create an image with 5x5 dimensions
            let mut img = DynamicImage::new_rgba8(5, 5);
            img.put_pixel(2, 2, Rgba([50, 50, 50, 50]));

            // Store image with dimensions 10x10
            let img_ref = cache.store("test", img.clone(), 10, 10);

            // Check if the image was resized correctly
            let dimensions = img_ref.dimensions();
            assert_eq!(dimensions, (10, 10));

            // Drop the first reference before creating the second
            std::mem::drop(img_ref);

            // Check if the same reference is returned for an existing key
            let img_ref_second = cache.store("test", img, 10, 10);
            assert_eq!(img_ref_second.dimensions(), dimensions);
        }
    }
}
