use image::{DynamicImage, GenericImageView};
use pixels::Pixels;

// /// Creates an inset border on an image without changing its size.
// pub fn inset_border(mut image: DynamicImage, border_width: f32) -> DynamicImage {
//     let (width, height) = image.dimensions();
//     let bw = (border_width as u32).min(width / 2).min(height / 2);
//
//     let border_color = image::Rgba([255, 255, 255, 255]); // white color for the border
//
//     for y in 0..height {
//         for x in 0..width {
//             if x < bw || x >= width - bw || y < bw || y >= height - bw {
//                 image.put_pixel(x, y, border_color);
//             }
//         }
//     }
//
//     image
// }

pub fn border(
    frame: &mut Pixels,
    frame_w: f32,
    frame_h: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    border_width: f32,
) {
    let bw = border_width.min(width / 2.0).min(height / 2.0);

    let start_x = x.max(0.0) as usize;
    let start_y = y.max(0.0) as usize;
    let end_x = (x + width).min(frame_w) as usize;
    let end_y = (y + height).min(frame_h) as usize;
    let bw_usize = bw as usize;

    let pixels = frame.frame_mut();

    for j in start_y..end_y {
        for i in start_x..end_x {
            if (i - start_x) < bw_usize
                || (end_x - i) <= bw_usize
                || (j - start_y) < bw_usize
                || (end_y - j) <= bw_usize
            {
                let index = (j * frame_w as usize + i) * 4;
                pixels[index] = 255; // red
                pixels[index + 1] = 255; // green
                pixels[index + 2] = 255; // blue
                pixels[index + 3] = 255; // alpha
            }
        }
    }
}

/// Copies data to a pixelbuffer at an offset without resizing.
pub fn copy_with_offset(
    image: &DynamicImage,
    frame: &mut Pixels,
    frame_w: u32,
    frame_h: u32,
    offset_x: u32,
    offset_y: u32,
) {
    let (image_w, image_h) = image.dimensions();
    let image_pixels = image.to_rgba8();
    let frame_pixels = frame.frame_mut();

    for y in 0..image_h {
        for x in 0..image_w {
            let frame_x = offset_x.saturating_add(x);
            let frame_y = offset_y.saturating_add(y);

            // Only continue if the frame_x and frame_y are within the frame bounds
            if frame_x < frame_w && frame_y < frame_h {
                let frame_idx = (frame_y * frame_w + frame_x) as usize * 4;

                // Only copy pixel if the indices are within the respective bounds
                if frame_idx + 3 < frame_pixels.len() {
                    let rgba = image_pixels.get_pixel(x, y);
                    frame_pixels[frame_idx..frame_idx + 4].copy_from_slice(&rgba.0);
                }
            }
        }
    }
}

pub fn clear(pixels: &mut Pixels) {
    let frame_pixels = pixels.frame_mut();

    for pixel in frame_pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, 255]);
    }
}

/// Copies data to a pixelbuffer, resizing to fit the size of the target buffer.
pub fn copy_and_resize(image: &DynamicImage, pixels: &mut Pixels, width: u32, height: u32) {
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
    let frame_pixels = pixels.frame_mut();

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
            frame_pixels[(position * 4)..(position * 4 + 4)].copy_from_slice(&rgba);
        }
    }
}
