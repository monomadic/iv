use crate::{error::IVError, filesystem};

#[derive(Default, Clone, Debug)]
pub struct AssetCollection {
    pub keys: Vec<String>,
    pub cursor: usize,
}

impl TryFrom<&str> for AssetCollection {
    type Error = IVError;

    fn try_from(p: &str) -> Result<Self, Self::Error> {
        Self::new(p)
    }
}

impl AssetCollection {
    pub fn new(path: &str) -> Result<Self, IVError> {
        // ensure path is standardised + absolute
        let path = std::fs::canonicalize(path)?;
        let keys = filesystem::get_images_from_dir(&path).expect("could not read dir");
        let mut collection = AssetCollection { keys, cursor: 0 };

        if path.is_file() {
            collection.set_current(path.to_str().expect("path to convert"));
        };

        Ok(collection)
    }

    pub fn set_current(&mut self, current: &str) {
        // find the current index
        let index = self.keys.iter().position(|p| p == current);
        self.cursor = index.expect(&format!(
            "attempted to set cursor to unknown image: {}",
            current
        ));
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

    pub fn move_to_beginning(&mut self) {
        self.cursor = 0;
    }

    pub fn move_to_end(&mut self) {
        self.cursor = self.keys.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_set_current() {
        let mut collection = AssetCollection {
            keys: vec![String::from("images/1.jpg"), String::from("images/2.png")],
            cursor: 0,
        };
        collection.set_current("images/2.png");
        assert_eq!(collection.current(), "images/2.png");
    }
}
