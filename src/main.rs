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
mod single_view;
mod window;

pub use collection::simple::AssetCollection;
pub(crate) use window::Window;

use std::env;

fn main() {
    // deal with cli
    let path = env::args().nth(1).unwrap_or("assets".into());
    //let assets = glob_from_arg(&path).expect("glob fail");

    let assets = loader::parse_arg(&path).expect("path fail");

    // asset cache
    let collection = AssetCollection { assets };

    // show window
    Window::new(collection);
}
