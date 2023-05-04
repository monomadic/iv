pub struct Config {
    pub thumbnail_padding: u32,
    // thumbnail_border_color: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thumbnail_padding: 6,
        }
    }
}
