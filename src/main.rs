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

fn main() -> Result<(), error::IVError> {
    // Get default configuration
    let config = config::Config::default();

    // Parse cli arguments
    let path = std::env::args().nth(1).unwrap_or(".".into());

    // Initialize application state
    let state = state::AppState::new(path, config.index_columns)?;

    // Create the initial UI
    let layout = components::AppComponent::default();

    // Show application window
    window::Window::new(state, layout, config)
}
