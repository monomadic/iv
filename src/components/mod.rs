use pixels::Pixels;

use crate::{config::Config, msg::Msg, state::AppState};

mod app_component;
mod gallery_component;
mod image_component;
mod index_view;
mod solo_view;

pub use app_component::AppComponent;
pub use index_view::IndexView;

// TODO: draw should contain the width + height?
// - prevents natural caching
// - but creates natural immutable component state

pub trait Component {
    fn draw(&mut self, state: &AppState, config: &Config, pixels: &mut Pixels);
    fn update(&mut self, state: &mut AppState, config: &Config, msg: &Msg) -> bool;
    // fn children(&self) -> Vec<&Self>;
}
