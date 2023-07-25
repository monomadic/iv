use pixels::Pixels;

use crate::{config::Config, msg::Msg, state::AppState};

mod app_component;
mod gallery_component;
mod image_component;
mod index_view;
mod solo_view;

pub use app_component::AppComponent;
pub use index_view::IndexView;

pub trait Component {
    fn update(&mut self, msg: &Msg, state: &mut AppState, config: &Config) -> bool;
    fn draw(&mut self, state: &AppState, config: &Config, pixels: &mut Pixels);
    // fn children(&self) -> Vec<&Self>;
}
