// https://github.com/parasyte/pixels/blob/main/examples/minimal-winit/src/main.rs
// use fast_image_resize as fir;

use crate::prelude::*;
use image::DynamicImage;
use softbuffer::GraphicsContext;
use std::{collections::HashMap, path::PathBuf, sync::mpsc, thread};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::AssetCollection;

pub struct Window;

struct UpdateState {
    image: DynamicImage,
    path: PathBuf,
}

impl UpdateState {
    fn load(path: PathBuf) -> Result<Self> {
        image::open(&path)
            .map(|image| UpdateState {
                image,
                path: path.clone(),
            })
            .map_err(|e| FBIError::Generic(format!("update state failure: {:?}, {:?}", path, e)))
    }
}

impl Window {
    pub fn new(mut collection: AssetCollection) -> Result<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("fbi")
            .with_decorations(false)
            .build(&event_loop)
            .expect("winit failed to initialize window");

        let mut decorations = true;
        let mut cache: HashMap<PathBuf, DynamicImage> = HashMap::new();

        let mut gallery_rows = 4;

        let mut single_view = false;

        // go fullscreen
        window.set_simple_fullscreen(true);

        let (width, height): (u16, u16) = window.inner_size().into();

        // create screen buffer (black screen)
        let mut screen_buffer = vec![0; width as usize * height as usize];
        let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

        graphics_context.set_buffer(&screen_buffer, width as u16, height as u16);
        //window.request_redraw();

        let (tx, rx) = mpsc::channel();

        // Preload the first image using mpsc channel
        let preload_tx = tx.clone();

        // preload images in a new thread
        {
            let collection = collection.clone();
            thread::spawn(move || {
                for path in collection.assets {
                    preload_tx.send(UpdateState::load(path)).unwrap();
                }
            });
        }

        event_loop.run(move |event, _elwt, control_flow| {
            control_flow.set_wait();

            use VirtualKeyCode::*;

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    // create a screen-sized dynamic view
                    let view = DynamicImage::new_rgb8(width as u32, height as u32);

                    let layout = if single_view {
                        let path = collection.current().unwrap();
                        let image = cache.get(path).unwrap();
                        crate::layout::render_single_view(image, view).expect("abc")
                    } else {
                        let images: Vec<&DynamicImage> = cache.iter().map(|(_k, v)| v).collect();
                        // crate::layout::render_multi_view(images, view, gallery_rows).expect("abc")
                        crate::layout::render_index_view(images, view, gallery_rows)
                            .expect("index view error")
                    };

                    screen_buffer = crate::layout::image_to_u32(layout);
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
                        Escape | Q => control_flow.set_exit(),
                        VirtualKeyCode::Key1 => {
                            gallery_rows = 3;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key2 => {
                            gallery_rows = 4;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Key3 => {
                            gallery_rows = 5;
                            window.request_redraw();
                        }
                        VirtualKeyCode::Space => {
                            single_view = !single_view;
                            window.request_redraw();
                        }
                        VirtualKeyCode::F => {
                            window.set_simple_fullscreen(!window.simple_fullscreen());
                        }
                        VirtualKeyCode::D => {
                            decorations = !decorations;
                            window.set_decorations(decorations);
                        }
                        VirtualKeyCode::K | VirtualKeyCode::H => {
                            collection.prev();
                            window.request_redraw();
                        }
                        VirtualKeyCode::J | VirtualKeyCode::L => {
                            collection.next();
                            window.request_redraw();
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => {}
            }

            if let Ok(result) = rx.try_recv() {
                // unwrap state result
                let new_state = result.expect("image failed to render");

                cache.insert(new_state.path, new_state.image);

                window.request_redraw();
            }
        });
    }
}
