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

// multi view
// takes a collection
// static view at first iterating entire collection of loaded objects

pub fn render_single_view(image: DynamicImage, width: u32, height: u32) -> Vec<u32> {
    let image = image.resize(width, height, FilterType::Lanczos3);

    // create a screen-sized buffer
    let mut screen = DynamicImage::new_rgb8(width as u32, height as u32);

    // align horizontal center by calculating left offset
    let left_offset = width / 2 - image.width() / 2;

    screen
        .copy_from(&image, left_offset as u32, 0)
        .expect("screen copy fail");

    image_to_u32(screen)
}

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
