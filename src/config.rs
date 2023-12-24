use winit::event::VirtualKeyCode;

pub struct Config {
    pub thumbnail_padding: u32,
    pub thumbnail_border_thickness: u32,
    // thumbnail_border_color: u32,
    // thumbnail_aspect: ThumbnailAspect, // whether to display thumbnails using the full space of the thumbnail
    pub index_columns: u32,
    pub keymaps: Keymaps,
    // pub fullscreen: bool,
}

pub struct Keymaps {
    pub index_goto_end: VirtualKeyCode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thumbnail_padding: 6,
            thumbnail_border_thickness: 10,
            index_columns: 6,
            keymaps: Keymaps::default(),
        }
    }
}

impl Default for Keymaps {
    fn default() -> Self {
        Keymaps {
            index_goto_end: VirtualKeyCode::G,
        }
    }
}
