use pixels::Pixels;

use crate::{config::Config, msg::Msg, state::AppState};

mod app;
mod index_view;

pub use app::App;
pub use index_view::IndexView;

// pub type AppComponent = dyn Component<AppState>;

pub trait Component {
    fn resize(&mut self, width: u32, height: u32);
    // TODO: make Msg and Appstate generic <M, S>
    fn update(&mut self, msg: Msg, state: &mut AppState) -> bool;
    // TODO: remove config (use state), make pixels -> renderer, make generic <R>
    fn draw(&mut self, state: &AppState, config: &Config, pixels: &mut Pixels);
}
