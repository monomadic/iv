#![allow(dead_code)]

mod app;
mod buffer;
// mod cache;
mod collection;
mod error;
mod layout;
mod loader;
mod prelude;
mod renderer;
mod window;

use app::AppState;
pub use collection::simple::AssetCollection;
pub(crate) use window::Window;

use crate::prelude::*;
use std::env;

fn main() -> Result<()> {
    // parse args
    let path = env::args().nth(1).unwrap_or("assets".into());
    // init state
    let appstate = AppState::new(path)?;
    // show window
    Window::new(appstate)
}
