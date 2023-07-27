#[derive(Default, Clone, Debug)]
pub struct AssetCollection {
    pub keys: Vec<String>,
    pub cursor: usize,
}

impl TryFrom<&str> for AssetCollection {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        // let files = crate::filesystem::get_files_from_directory(path)?;
        let keys = crate::filesystem::get_images_from_dir(path).expect("could not read dir");

        Ok(Self { keys, cursor: 0 })
    }
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
