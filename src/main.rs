mod app;
mod collection;
mod config;
mod error;
mod loader;
mod prelude;
mod render;
mod window;

use crate::prelude::*;

fn main() -> Result<()> {
    // default config
    let config = config::Config::default();
    // parse args
    let path = std::env::args().nth(1).unwrap_or(".".into());
    // init state
    let appstate = app::AppState::new(path)?;
    // show window
    window::Window::new(appstate, config)
}
