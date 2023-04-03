#![allow(dead_code)]

mod app;
mod buffer;
mod cache;
mod collection;
mod cyclevec;
mod error;
mod layout;
mod loader;
mod prelude;
mod single_view;
mod window;

pub use collection::simple::AssetCollection;
pub(crate) use window::Window;

use crate::prelude::*;
use std::env;

fn main() -> Result<()> {
    // deal with cli
    let path = env::args().nth(1).unwrap_or("assets".into());
    let paths = loader::parse_arg(&path)?;

    // asset cache
    let collection = AssetCollection::new(paths);

    // show window
    Window::new(collection)
}
