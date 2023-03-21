use std::num::NonZeroU32;

pub struct ScreenBuffer {
    width: NonZeroU32,
    height: NonZeroU32,
    pub buffer: Vec<u32>,
}

impl ScreenBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: NonZeroU32::new(width).expect("zero value width"),
            height: NonZeroU32::new(height).expect("zero value height"),
            buffer: Vec::new(),
        }
    }
}
