use pixels::{Pixels, SurfaceTexture};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    components::{App, Component},
    config::Config,
    msg::Msg,
    prelude::*,
    state::AppState,
};

pub struct Window;

impl Window {
    pub fn new(mut state: AppState, mut app: App, config: Config) -> Result<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("iV")
            .with_decorations(false)
            .build(&event_loop)
            .expect("winit failed to initialize window");

        // go fullscreen
        window.set_simple_fullscreen(true);

        let (width, height): (u32, u32) = window.inner_size().into();
        app.resize(width, height);

        let mut pixels = {
            let surface_texture = SurfaceTexture::new(width, height, &window);
            Pixels::new(width, height, surface_texture)
        }
        .expect("pixels err"); // TODO: coalesce
        pixels.clear_color(pixels::wgpu::Color::BLACK);

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            use VirtualKeyCode::*;

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    app.draw(&state, &config, &mut pixels);

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
                            state.cols = 3;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key2 => {
                            state.cols = 4;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key3 => {
                            state.cols = 5;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key4 => {
                            state.cols = 6;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key5 => {
                            state.cols = 7;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key6 => {
                            state.cols = 8;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key7 => {
                            state.cols = 9;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Minus => {
                            state.cols += 1;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Equals => {
                            if state.cols > 2 {
                                state.cols -= 1;
                            }
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key8 => {
                            state.cols = 10;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Space | VirtualKeyCode::Return => {
                            state.toggle_layout();
                            window.request_redraw();
                        }
                        VirtualKeyCode::F => {
                            window.set_simple_fullscreen(!window.simple_fullscreen());
                        }
                        VirtualKeyCode::H | VirtualKeyCode::Left => {
                            if app.update(Msg::MoveLeft, &mut state) {
                                window.request_redraw();
                            }
                        }
                        VirtualKeyCode::L | VirtualKeyCode::Right => {
                            if app.update(Msg::MoveRight, &mut state) {
                                window.request_redraw();
                            }
                        }
                        VirtualKeyCode::J | VirtualKeyCode::Down => {
                            if app.update(Msg::MoveDown, &mut state) {
                                window.request_redraw();
                            }
                        }
                        VirtualKeyCode::K | VirtualKeyCode::Up => {
                            if app.update(Msg::MoveUp, &mut state) {
                                window.request_redraw();
                            }
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
