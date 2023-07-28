use pixels::Pixels;
use winit::dpi::PhysicalSize;

use crate::{msg::Msg, state::AppState};

mod app_component;
mod image_component;
mod index_view;

pub use app_component::AppComponent;
pub use index_view::IndexView;

pub struct Rect {
    pub width: f32,
    pub height: f32,
}

impl From<PhysicalSize<u32>> for Rect {
    fn from(size: PhysicalSize<u32>) -> Self {
        Rect {
            width: size.width as f32,
            height: size.height as f32,
        }
    }
}

pub trait Component {
    fn draw(&mut self, state: &AppState, size: &Rect, pixels: &mut Pixels);
    fn update(&mut self, state: &mut AppState, size: &Rect, msg: &Msg) -> bool;
}
