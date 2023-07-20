mod cache;
mod components;
mod config;
mod error;
mod filesystem;
mod image;
mod msg;
mod prelude;
mod state;
mod window;

use crate::prelude::*;

fn main() -> Result<()> {
    let config = config::Config::default();
    // parse args
    let path = std::env::args().nth(1).unwrap_or(".".into());
    // init state
    let state = state::AppState::new(path)?;
    // layout
    let layout = components::AppComponent::default();
    // show window
    window::Window::new(state, layout, config)
}
