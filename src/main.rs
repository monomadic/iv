mod error;
mod filesystem;
mod prelude;
mod window;

use image::GenericImageView;
use std::env;

use crate::{filesystem::FileCollection, window::Window};

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        // panic!("Please enter a file")
        "assets/*".into()
    };

    let mut collection = FileCollection::from_glob(&file).unwrap();

    let path = collection.next().unwrap();

    // Open the JPEG file
    let image = image::open(path).unwrap();
    let mut width = image.width() as usize;
    let mut height = image.height() as usize;
    println!("dimensions {:?} {:?}", image.dimensions(), image.color());

    let mut u32_buffer: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    Window::new(collection);

    // // Create the window
    // let mut window = Window::new(
    //     "fbi",
    //     width,
    //     height,
    //     WindowOptions {
    //         borderless: true,
    //         ..WindowOptions::default()
    //     },
    // )
    // .unwrap();
    //
    // // Loop until the window is closed
    // while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
    //     if window.is_key_pressed(Key::J, KeyRepeat::No) {
    //         let image = image::open(collection.next().unwrap()).unwrap();
    //         width = image.width() as usize;
    //         height = image.height() as usize;
    //
    //         u32_buffer = image
    //             .as_bytes()
    //             .chunks(3)
    //             .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
    //             .collect();
    //     }
    //     // Draw the image to the window
    //     window
    //         .update_with_buffer(u32_buffer.as_slice(), width, height)
    //         .unwrap();
    // }
}
