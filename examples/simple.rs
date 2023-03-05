use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use std::{env, path::Path};

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        // panic!("Please enter a file")
        "assets/iso.jpg".into()
    };

    // Open the JPEG file
    let image = image::open(&Path::new(&file)).unwrap();
    let width = image.width() as usize;
    let height = image.height() as usize;
    println!("dimensions {:?} {:?}", image.dimensions(), image.color());

    let u32_buffer: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    // Create the window
    let mut window = Window::new(
        "fbi",
        width,
        height,
        WindowOptions {
            borderless: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // Loop until the window is closed
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        // Draw the image to the window
        window
            .update_with_buffer(u32_buffer.as_slice(), width, height)
            .unwrap();
    }
}
