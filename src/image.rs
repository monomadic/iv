use image::{DynamicImage, GenericImageView};
use pixels::Pixels;

pub fn copy_image(image: &DynamicImage, pixels: &mut Pixels, width: u32, height: u32) {
    let (image_width, image_height) = image.dimensions();

    let image_aspect_ratio = image_width as f32 / image_height as f32;
    let frame_aspect_ratio = width as f32 / height as f32;

    let (new_width, new_height) = if image_aspect_ratio > frame_aspect_ratio {
        (width, (width as f32 / image_aspect_ratio) as u32)
    } else {
        ((height as f32 * image_aspect_ratio) as u32, height)
    };

    let x_offset = (width - new_width) / 2;
    let y_offset = (height - new_height) / 2;

    let pixels_frame = pixels.frame_mut();

    // black image
    for pixel in pixels_frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, 255]);
    }

    for y in 0..height {
        for x in 0..width {
            // pixel out of bounds
            if x < x_offset || x >= width - x_offset || y < y_offset || y >= height - y_offset {
                continue;
            }

            let source_x = ((x - x_offset) as f32 * (image_width as f32 / new_width as f32)) as u32;
            let source_y =
                ((y - y_offset) as f32 * (image_height as f32 / new_height as f32)) as u32;

            // Clamp the source_x and source_y values within valid bounds
            let clamped_source_x = source_x.min(image_width - 1);
            let clamped_source_y = source_y.min(image_height - 1);

            let pixel = image.get_pixel(clamped_source_x, clamped_source_y);
            let rgba = pixel.0;

            let position = ((y * width) + x) as usize;
            // Each pixel has 4 channels (RGBA), so we multiply the position by 4.
            pixels_frame[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
        }
    }
}
