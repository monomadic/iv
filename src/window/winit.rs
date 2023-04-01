// https://github.com/parasyte/pixels/blob/main/examples/minimal-winit/src/main.rs

// use std::num::NonZeroU32;
// use fast_image_resize as fir;

use std::{sync::mpsc, thread};

use softbuffer::GraphicsContext;

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{layout::render_single_view, AssetCollection};

pub struct Window;

impl Window {
    pub fn new(mut collection: AssetCollection) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("fbi")
            .with_decorations(false)
            .build(&event_loop)
            .expect("winit failed to initialize window");

        let mut decorations = true;

        // go fullscreen
        window.set_simple_fullscreen(true);

        let (width, height): (u16, u16) = window.inner_size().into();

        // create screen buffer (black screen)
        let mut screen_buffer = vec![0; (width as usize * height as usize)];
        let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

        graphics_context.set_buffer(&screen_buffer, width as u16, height as u16);

        let (tx, rx) = mpsc::channel();

        // Preload the first image using mpsc channel
        let preload_tx = tx.clone();
        let image = collection.next().unwrap();
        thread::spawn(move || {
            preload_tx.send(AssetCollection::process(image)).unwrap();
        });

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    graphics_context.set_buffer(&screen_buffer, width as u16, height as u16);
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
                            // let image = collection.next().unwrap();
                            // screen_buffer = render_single_view(image, width as u32, height as u32);
                            // window.request_redraw();

                            let next = collection.next().expect("ooooop");

                            let result = AssetCollection::process(next);
                            // Send the result back to the main thread.
                            tx.send(result).unwrap();
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => {}
            }

            if let Ok(result) = rx.try_recv() {
                let image = result.expect("image failed to render");
                screen_buffer = render_single_view(image, width as u32, height as u32);
                window.request_redraw();
            }
        });
    }
}
