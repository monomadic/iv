use pixels::{Pixels, SurfaceTexture};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::{
    event::{ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    components::{AppComponent, Component},
    msg::Msg,
    prelude::*,
    state::AppState,
};

pub struct Window;

impl Window {
    pub fn new(mut state: AppState, mut app: AppComponent) -> Result<()> {
        let event_loop = EventLoop::new();
        // Keyboard modifier state
        let mut modifiers = ModifiersState::default();
        let window = WindowBuilder::new()
            .with_title("iv")
            .with_decorations(false)
            .build(&event_loop)
            .expect("winit failed to initialize window");

        // Go fullscreen
        window.set_simple_fullscreen(true);

        let mut size = window.inner_size().into();
        app.update(&mut state, &size, &Msg::Init);

        // Initialize Pixels
        let mut pixels = {
            let surface_texture =
                SurfaceTexture::new(size.width as u32, size.height as u32, &window);
            Pixels::new(size.width as u32, size.height as u32, surface_texture)?
        };
        pixels.clear_color(pixels::wgpu::Color::BLACK);

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            use VirtualKeyCode::*;

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    app.draw(&mut state, &size, &mut pixels);

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::ModifiersChanged(new) => {
                        modifiers = new;
                    }
                    WindowEvent::Resized(new_size) => {
                        size = new_size.into();
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        match virtual_code {
                            Escape | Q => control_flow.set_exit(),
                            VirtualKeyCode::F => {
                                window.set_simple_fullscreen(!window.simple_fullscreen());
                            }
                            _ => (),
                        };

                        if app.update(&mut state, &size, &Msg::KeyPress(virtual_code, modifiers)) {
                            window.request_redraw();
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        });
    }
}
