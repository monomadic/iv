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
    // Parse cli arguments
    // TODO: switch to clap to accept the path argument
    // TODO: add a flag to start in gallery mode
    // TODO: add a flag to start in fullscreen mode
    let path: &str = &std::env::args().nth(1).unwrap_or(".".into());

    // Determine whether to start in gallery or single mode
    let layout_state = state::LayoutState::from(path);

    // Create a collection of images from the input path
    let collection = state::AssetCollection::try_from(path)?;

    // Get default configuration
    let config = config::Config::default();

    // Initialize application state
    let state = state::AppState::new(layout_state, collection, config)?;

    // Create the initial UI
    let layout = components::AppComponent::default();

    // Show application window
    window::Window::new(state, layout)
}
