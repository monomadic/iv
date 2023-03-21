#![allow(dead_code)]

mod cache;
mod collection;
mod cyclevec;
mod error;
// mod filesystem;
mod app;
mod buffer;
mod layout;
mod loader;
mod prelude;
mod window;

pub use collection::simple::AssetCollection;
use loader::path_from_args;
pub(crate) use window::Window;

use std::env;

fn main() {
    // deal with cli
    let path = env::args().nth(1).unwrap_or("assets/*".into());
    let input_paths = path_from_args(&path).expect("path fail");
    // crate asset cache
    let collection = AssetCollection::new(input_paths);

    // show window
    Window::new(collection);
}
