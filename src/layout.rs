use crate::prelude::*;
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView};

#[derive(Default)]
pub enum LayoutState {
    #[default]
    SingleView,
    MultiView,
}

impl LayoutState {
    pub fn draw(&self) -> Vec<u32> {
        todo!()
    }
}

pub fn render_index_view(
    images: Vec<&DynamicImage>,
    mut surface: DynamicImage,
    columns: u32,
) -> Result<DynamicImage> {
    let col_width = surface.width() / columns;
    let row_height = col_width; // square grid

    let rows = (surface.height() as f64 / row_height as f64).floor() as u32; // todo: use ceil,
                                                                             // render half-images

    // maximum amount of images that can fit on the screen
    let max_images = rows * columns;

    println!("{}, {}, {}", rows, columns, max_images);

    // let horizontal_spacing = (surface.width() - (columns * row_height)) / (columns + 1);
    // let vertical_spacing = (surface.height() - (rows * row_height)) / (rows + 1);

    for (i, image) in images.iter().enumerate() {
        println!("i:{}", i);
        if i as u32 >= max_images {
            break;
        }

        let resized_image = image.resize(row_height, row_height, FilterType::Lanczos3);

        let current_row = (i as u32) / columns;
        let current_column = (i as u32) % columns;

        let left_padding = current_column * (row_height);
        let top_padding = (current_row * (row_height));

        surface
            .copy_from(&resized_image, left_padding, top_padding)
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
    // let surface_width = surface.width();

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
        .copy_from(&image, left_offset as u32, 0)
        .map_err(|e| FBIError::Generic(e.to_string()))?;

    Ok(surface)
}

pub fn image_to_u32(img: DynamicImage) -> Vec<u32> {
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
