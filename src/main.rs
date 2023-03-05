extern crate image;
extern crate minifb;

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
    let im = image::open(&Path::new(&file)).unwrap();

    println!("dimensions {:?} {:?}", im.dimensions(), im.color());
    let (width, height) = (im.dimensions().0 as usize, im.dimensions().1 as usize);

    let u32_buffer: Vec<u32> = im
        .as_bytes()
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    // Create the window
    let mut window = Window::new("fbi", width, height, WindowOptions::default()).unwrap();

    // Loop until the window is closed
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        // Draw the image to the window
        window
            .update_with_buffer(u32_buffer.as_slice(), width, height)
            .unwrap();
    }
}
