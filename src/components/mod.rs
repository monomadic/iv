use pixels::Pixels;

use crate::{config::Config, msg::Msg, state::AppState};

mod app;
mod index_view;

pub use app::App;
pub use index_view::IndexView;

pub trait Component {
    fn resize(&mut self, width: u32, height: u32);
    fn update(&mut self, msg: Msg, state: &mut AppState) -> bool;
    fn draw(&mut self, state: &AppState, config: &Config, pixels: &mut Pixels);
}
