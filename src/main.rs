#![allow(dead_code)]

mod app;
mod asset_collection;
mod error;
mod layout;
mod loader;
mod prelude;
mod rendercache;
mod window;

use crate::prelude::*;

fn main() -> Result<()> {
    // parse args
    let path = std::env::args().nth(1).unwrap_or(".".into());
    // init state
    let appstate = app::AppState::new(path)?;
    // show window
    window::Window::new(appstate)
}
