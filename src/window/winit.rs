// https://github.com/parasyte/pixels/blob/main/examples/minimal-winit/src/main.rs

use softbuffer::GraphicsContext;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::filesystem::FileCollection;

pub struct Window;

impl Window {
    pub fn new(mut collection: FileCollection) {
        let event_loop = EventLoop::new();

        let mut decorations = true;

        let window = WindowBuilder::new()
            .with_title("fbi")
            .build(&event_loop)
            .unwrap();

        window.set_decorations(false);
        window.set_simple_fullscreen(true);
        collection.next();

        let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    println!("redraw req");
                    let buffer = collection.current();
                    graphics_context.set_buffer(
                        &buffer.buffer,
                        buffer.width as u16,
                        buffer.height as u16,
                    );
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
                        VirtualKeyCode::Escape | VirtualKeyCode::Q => control_flow.set_exit(),
                        VirtualKeyCode::F => {
                            window.set_simple_fullscreen(!window.simple_fullscreen());
                        }
                        VirtualKeyCode::D => {
                            decorations = !decorations;
                            window.set_decorations(decorations);
                        }
                        VirtualKeyCode::J => {
                            println!("j hit");
                            collection.next();
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
