use winit::event::VirtualKeyCode;

use crate::{msg::Msg, state::AppState};

use super::Component;

#[derive(Default)]
pub struct ImageComponent {
    width: u32,
    height: u32,
    zoom: Zoom,
}

#[derive(Default)]
pub enum Zoom {
    #[default]
    FitToScreen,
    ZoomToFit,
    Zoom(f32),
}

impl Component for ImageComponent {
    fn update(&mut self, state: &mut AppState, msg: &Msg) -> bool {
        match msg {
            Msg::Resized(width, height) => {
                self.width = *width;
                self.height = *height;
            }
            Msg::MoveLeft | Msg::MoveUp => {
                state.collection.decrement(1);
            }
            Msg::MoveRight | Msg::MoveDown => {
                state.collection.increment(1);
            }
            Msg::KeyPress(key, _modifiers) => match key {
                VirtualKeyCode::Key1 => {
                    self.zoom = Zoom::Zoom(1.0);
                }
                VirtualKeyCode::Key2 => {
                    self.zoom = Zoom::Zoom(2.0);
                }
                VirtualKeyCode::Key3 => {
                    self.zoom = Zoom::Zoom(3.0);
                }
                VirtualKeyCode::Minus => {
                    self.zoom = Zoom::Zoom(0.5);
                }
                VirtualKeyCode::Key0 => {
                    self.zoom = Zoom::FitToScreen;
                }
                VirtualKeyCode::A => {
                    self.zoom = match self.zoom {
                        Zoom::FitToScreen => Zoom::ZoomToFit,
                        Zoom::ZoomToFit => Zoom::FitToScreen,
                        Zoom::Zoom(_) => Zoom::ZoomToFit,
                    }
                }
                _ => (),
            },
        }
        true
    }

    fn draw(&mut self, state: &crate::state::AppState, pixels: &mut pixels::Pixels) {
        match self.zoom {
            Zoom::FitToScreen => {
                crate::image::clear(pixels);
                crate::image::copy_and_resize(
                    state.current_image(),
                    pixels,
                    self.width,
                    self.height,
                );
            }
            Zoom::ZoomToFit => {
                todo!();
            }
            Zoom::Zoom(zoom) => {
                crate::image::clear(pixels);
                let image = state.current_image();

                if zoom == 1.0 {
                    let offset_x = self.width / 2 - image.width() / 2;
                    let offset_y = self.height / 2 - image.height() / 2;

                    crate::image::copy_with_offset(
                        &image,
                        pixels,
                        self.width,
                        self.height,
                        offset_x,
                        offset_y,
                    );
                } else {
                    let width = (image.width() as f32 * zoom) as u32;
                    let height = (image.height() as f32 * zoom) as u32;
                    let offset_x = self.width / 2 - width / 2;
                    let offset_y = self.height / 2 - height / 2;

                    let image = image.resize(width, height, image::imageops::FilterType::Lanczos3);
                    crate::image::copy_with_offset(
                        &image,
                        pixels,
                        self.width,
                        self.height,
                        offset_x,
                        offset_y,
                    );
                }
            }
        }
    }
}
