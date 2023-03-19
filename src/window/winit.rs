// https://github.com/parasyte/pixels/blob/main/examples/minimal-winit/src/main.rs

use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView};
use softbuffer::GraphicsContext;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::AssetCollection;

pub struct Window;

fn image_to_u32(img: DynamicImage) -> Vec<u32> {
    let (img_width, img_height) = img.dimensions();
    let mut buffer: Vec<u32> = vec![];
    buffer.resize((img_width * img_height) as usize, 0);

    for y in 0..img_height {
        for x in 0..img_width {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;
            let color = ((rgba[3] as u32) << 24)
                | ((rgba[0] as u32) << 16)
                | ((rgba[1] as u32) << 8)
                | (rgba[2] as u32);
            buffer[y as usize * img_width as usize + x as usize] = color;
        }
    }

    buffer
}

impl Window {
    pub fn new(mut collection: AssetCollection) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("fbi")
            .with_decorations(false)
            .build(&event_loop)
            .unwrap();

        let mut decorations = true;

        // go fullscreen
        window.set_simple_fullscreen(true);
        let size = window.inner_size();
        let width = size.width as usize;
        let height = size.height as usize;

        // create screen buffer
        let mut screen_buffer = (0..((width * height) as usize))
            .map(|_| 0)
            .collect::<Vec<u32>>();

        let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
        graphics_context.set_buffer(&screen_buffer, width as u16, height as u16);

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
                            println!("j hit");
                            let image = collection.next().unwrap();
                            // let image =
                            //     image.resize(width as u32, height as u32, FilterType::Lanczos3);
                            // let image =
                            //     image::open("../_assets/girl.jpg").expect("Failed to open image");

                            let mut screen = DynamicImage::new_rgb8(width as u32, height as u32);
                            screen.copy_from(&image, 0, 0).unwrap();

                            screen_buffer = image_to_u32(screen);
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
