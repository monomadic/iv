use crate::prelude::*;
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView};

#[derive(Default)]
pub enum LayoutState {
    #[default]
    SingleView,
    MultiView,
}

pub fn render_index_view(
    images: Vec<&DynamicImage>,
    mut surface: DynamicImage,
    columns: u32,
    selected: usize,
) -> Result<DynamicImage> {
    let col_width = surface.width() / columns;
    let row_height = col_width; // square grid

    let rows = (surface.height() as f64 / row_height as f64).floor() as u32; // todo: use ceil,
                                                                             // render half-images

    // maximum amount of images that can fit on the screen
    let max_images = (rows * columns) as usize;

    // let horizontal_spacing = (surface.width() - (columns * row_height)) / (columns + 1);
    // let vertical_spacing = (surface.height() - (rows * row_height)) / (rows + 1);

    for (i, image) in images.iter().enumerate() {
        if i >= max_images {
            break;
        }

        let mut resized_image = image.resize(row_height, row_height, FilterType::Lanczos3);

        // if image is currently selected
        if i == selected {
            resized_image = resized_image.brighten(30);
        }

        let current_row = (i as u32) / columns;
        let current_column = (i as u32) % columns;

        let mut left_offset = current_column * (row_height);
        let mut top_offset = current_row * (row_height);

        // center image
        if image.height() > image.width() {
            // center horizontally
            left_offset = left_offset + ((col_width - resized_image.width()) / 2);
        } else {
            // center vertically
            top_offset = top_offset + ((col_width - resized_image.height()) / 2);
        }

        // apply padding here

        surface
            .copy_from(&resized_image, left_offset, top_offset)
            .map_err(|e| FBIError::Generic(e.to_string()))?;
    }

    Ok(surface)
}

pub fn render_multi_view(
    images: Vec<&DynamicImage>,
    mut surface: DynamicImage,
    rows: u32,
) -> Result<DynamicImage> {
    let row_height = surface.height() / rows;

    for (i, image) in images.iter().enumerate() {
        let image = image.resize(row_height, row_height, FilterType::Lanczos3);

        // align horizontal center by calculating left offset
        // pad + image_offset + centering_offset
        let left_padding = 20 + (i as u32 * row_height) + (row_height / 2 - image.width() / 2);
        let top_padding = 20;

        surface
            .copy_from(&image, left_padding, top_padding)
            .map_err(|e| FBIError::Generic(e.to_string()))?;
    }

    Ok(surface)
}

pub fn render_single_view(image: &DynamicImage, mut surface: DynamicImage) -> Result<DynamicImage> {
    let surface_height = surface.height();
    let surface_width = surface.width();

    let image = image.resize(surface_width, surface_height, FilterType::Lanczos3);

    // align horizontal center by calculating left offset
    let left_offset = surface_width / 2 - image.width() / 2;

    surface
        .copy_from(&image, left_offset, 0)
        .map_err(|e| FBIError::Generic(e.to_string()))?;

    Ok(surface)
}

pub fn _image_to_u32(img: DynamicImage) -> Vec<u32> {
    let (img_width, img_height) = img.dimensions();
    let img_rgba = img.into_rgba8();
    let mut buffer: Vec<u32> = Vec::with_capacity((img_width * img_height) as usize);

    for pixel in img_rgba.chunks_exact(4) {
        let color = ((pixel[3] as u32) << 24)
            | ((pixel[0] as u32) << 16)
            | ((pixel[1] as u32) << 8)
            | (pixel[2] as u32);
        buffer.push(color);
    }

    buffer
}

pub fn image_to_u32(img: DynamicImage) -> Vec<u32> {
    img.into_rgba8()
        .chunks_exact(4)
        .map(|pixel| {
            ((pixel[3] as u32) << 24)
                | ((pixel[0] as u32) << 16)
                | ((pixel[1] as u32) << 8)
                | (pixel[2] as u32)
        })
        .collect()
}
