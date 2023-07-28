use pixels::Pixels;

use crate::{msg::Msg, state::AppState};

mod app_component;
mod image_component;
mod index_view;

pub use app_component::AppComponent;
pub use index_view::IndexView;

// TODO: draw should contain the width + height?
// - prevents natural caching
// - but creates natural immutable component state

pub trait Component {
    fn draw(&mut self, state: &AppState, pixels: &mut Pixels);
    fn update(&mut self, state: &mut AppState, msg: &Msg) -> bool;
    // fn children(&self) -> Vec<&Self>;
}
