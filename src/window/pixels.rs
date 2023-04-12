use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;

use crate::prelude::*;
use crate::{app::AppState, rendercache::RenderCache};

pub struct Window;

impl Window {
    pub fn new(mut appstate: AppState) -> Result<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("iV")
            .with_decorations(false)
            .build(&event_loop)
            .expect("winit failed to initialize window");

        // go fullscreen
        window.set_simple_fullscreen(true);

        let (width, height): (u32, u32) = window.inner_size().into();

        let mut pixels = {
            let surface_texture = SurfaceTexture::new(width, height, &window);
            Pixels::new(width, height, surface_texture).expect("pixels err")
        };

        let mut render = RenderCache::init(width, height);

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            use VirtualKeyCode::*;

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    render.draw(&appstate, &mut pixels);

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match virtual_code {
                        Escape | Q => control_flow.set_exit(),
                        VirtualKeyCode::Key1 => {
                            appstate.cols = 3;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key2 => {
                            appstate.cols = 4;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key3 => {
                            appstate.cols = 5;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key4 => {
                            appstate.cols = 6;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key5 => {
                            appstate.cols = 7;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key6 => {
                            appstate.cols = 8;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key7 => {
                            appstate.cols = 9;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key8 => {
                            appstate.cols = 10;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Space | VirtualKeyCode::Return => {
                            appstate.toggle_layout();
                            window.request_redraw();
                        }
                        VirtualKeyCode::F => {
                            window.set_simple_fullscreen(!window.simple_fullscreen());
                        }
                        VirtualKeyCode::H | VirtualKeyCode::Left => {
                            appstate.left();
                            window.request_redraw();
                        }
                        VirtualKeyCode::J | VirtualKeyCode::Down => {
                            appstate.down();
                            window.request_redraw();
                        }
                        VirtualKeyCode::K | VirtualKeyCode::Up => {
                            appstate.up();
                            window.request_redraw();
                        }
                        VirtualKeyCode::L | VirtualKeyCode::Right => {
                            appstate.right();
                            window.request_redraw();
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => {}
            }
        });
    }
}
