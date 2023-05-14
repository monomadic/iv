use pixels::Pixels;

use crate::{config::Config, state::AppState};

mod app;
mod grid;

pub use app::App;
pub use grid::GridComponent;

pub trait Component {
    fn resize(&mut self, width: u32, height: u32);
    fn update(&mut self, state: &mut AppState) -> bool;
    fn draw(&mut self, state: &AppState, config: &Config, pixels: &mut Pixels);
}
