pub struct Config {
    pub thumbnail_padding: u32,
    pub thumbnail_border_thickness: u32,
    // thumbnail_border_color: u32,
    // thumbnail_aspect: ThumbnailAspect, // whether to display thumbnails using the full space of the thumbnail
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thumbnail_padding: 6,
            thumbnail_border_thickness: 10,
        }
    }
}
