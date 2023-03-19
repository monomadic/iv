mod cache;
mod collection;
mod cyclevec;
mod error;
// mod filesystem;
mod prelude;
mod window;

pub use collection::simple::AssetCollection;
pub(crate) use window::Window;

use std::{env, path::PathBuf};

fn main() {
    // deal with cli
    let path = env::args().nth(1).unwrap_or("assets/*".into());
    let input_paths: Vec<PathBuf> = glob::glob(&path)
        .unwrap()
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    // crate asset cache
    let collection = AssetCollection::new(input_paths);

    // show window
    Window::new(collection);
}
