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

        let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

        let path = collection.next().unwrap();

        let image = image::open(path).unwrap();
        let width = image.width() as usize;
        let height = image.height() as usize;

        let buffer: Vec<u32> = image
            .as_bytes()
            .chunks(3)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    // let (width, height) = {
                    //     let size = window.inner_size();
                    //     (size.width, size.height)
                    // };
                    // let buffer = (0..((width * height) as usize))
                    //     .map(|index| {
                    //         let y = index / (width as usize);
                    //         let x = index % (width as usize);
                    //         let red = x % 255;
                    //         let green = y % 255;
                    //         let blue = (x * y) % 255;
                    //
                    //         let color = blue | (green << 8) | (red << 16);
                    //
                    //         color as u32
                    //     })
                    //     .collect::<Vec<_>>();

                    graphics_context.set_buffer(&buffer, width as u16, height as u16);
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
                        _ => (),
                    },
                    _ => (),
                },
                _ => {}
            }
        });
    }
}
