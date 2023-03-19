use crate::prelude::*;
use std::{collections::HashMap, path::PathBuf};

/// Single threaded, just-in-time asset cache.
pub(crate) struct SimpleCache<T>
where
    T: TryInto<T>,
{
    cache: HashMap<PathBuf, T>,
}

impl<T: TryInto<T>> Default for SimpleCache<T> {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl<T: TryInto<T>> SimpleCache<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, path: PathBuf) {}

    /// Get an object from the cache.
    pub fn get(&mut self, path: PathBuf) -> Result<&T> {
        match self.cache.get(&path) {
            Some(item) => Ok(item),
            // None => Ok(&self.cache.insert(path, path.try_into()?).unwrap()),
            None => todo!(),
        }
    }
}
