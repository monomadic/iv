use std::path::PathBuf;

#[derive(Default, Clone, Debug)]
pub struct AssetCollection {
    pub keys: Vec<String>,
    // TODO: make private cursor, use a u32
    pub cursor: usize,
}

impl AssetCollection {
    pub fn new(keys: Vec<String>) -> Self {
        AssetCollection { keys, cursor: 0 }
    }

    pub fn set_current(&mut self, current: &str) {
        // find the current index
        let index = self.keys.iter().position(|p| p == current);
        self.cursor = index.expect("attempted to set cursor to unknown image");
    }

    pub fn current(&self) -> &str {
        self.keys
            .get(self.cursor)
            .expect("cursor is pointing to a path not included in the current collection")
    }

    pub fn increment(&mut self, inc: usize) {
        let next = self.cursor + inc;
        if next < self.keys.len() {
            self.cursor = next;
        } else {
            self.cursor = self.keys.len() - 1;
        }
    }

    pub fn decrement(&mut self, dec: usize) {
        if self.cursor > dec {
            self.cursor = self.cursor - dec;
        } else {
            self.cursor = 0;
        }
    }

    // pub fn thumbs(&self) -> Vec<&DynamicImage> {
    //     self.current_collection
    //         .iter()
    //         .map(|path| {
    //             // for now, returning originals
    //             self.current_collection
    //                 .get(path)
    //                 .unwrap_or(&self.placeholder)
    //             // let hash = self.hash(&path, thumb_width);
    //             // if let Some(cached_thumb) = self.cache.get(&hash) {
    //             //     Some(cached_thumb.clone())
    //             // } else {
    //             //     let processed_thumb =
    //             //         process_image(path, thumb_width, config.thumbnail_padding)?;
    //             //     self.cache.insert(hash, processed_thumb.clone());
    //             //     Some(processed_thumb)
    //             // }
    //         })
    //         .collect()
    // }

    // /// Get the next asset
    // pub fn prev(&mut self) -> Option<&PathBuf> {
    //     if self.cursor == 0 {
    //         self.cursor = self.assets.len() - 1;
    //     } else {
    //         self.cursor -= 1;
    //     }
    //
    //     self.assets.get(self.cursor)
    // }
    //
    // /// Get the next asset
    // pub fn next(&mut self) -> Option<&PathBuf> {
    //     if self.cursor == self.assets.len() - 1 {
    //         self.cursor = 0;
    //     } else {
    //         self.cursor += 1;
    //     }
    //
    //     self.assets.get(self.cursor)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_set_current() {
        let mut collection = AssetCollection::new(vec![
            String::from("images/1.jpg"),
            String::from("images/2.png"),
        ]);
        collection.set_current("images/2.png");
        assert_eq!(collection.current(), "images/2.png");
    }
}
