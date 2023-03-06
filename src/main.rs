mod error;
mod filesystem;
mod prelude;
mod window;

use crate::{filesystem::FileCollection, window::Window};
use std::env;

fn main() {
    let file = env::args().nth(1).unwrap_or("assets/*".into());
    let collection = FileCollection::from_glob(&file).unwrap();

    Window::new(collection);
}
