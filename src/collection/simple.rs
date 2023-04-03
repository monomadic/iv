use std::path::PathBuf;

#[derive(Default)]
pub struct AssetCollection {
    // assets: CycleVec<PathBuf>,
    pub assets: Vec<PathBuf>,
    //cache: HashMap<PathBuf, DynamicImage>,
    pub cursor: usize,
}

impl AssetCollection {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            assets: paths,
            // cache: HashMap::new(),
            cursor: 0,
        }
    }

    /// Get the next asset
    pub fn prev(&mut self) -> Option<&PathBuf> {
        if self.cursor == 0 {
            self.cursor = self.assets.len() - 1;
        } else {
            self.cursor -= 1;
        }

        self.assets.get(self.cursor)
    }

    /// Get the next asset
    pub fn next(&mut self) -> Option<&PathBuf> {
        if self.cursor == self.assets.len() - 1 {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }

        self.assets.get(self.cursor)
    }
}
