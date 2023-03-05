use minifb::Window as MiniFB;

pub struct Window;

impl Window {
    // maybe replace filecollection later
    pub fn new() -> Result<Self> {
        // Create the window
        let mut window = MiniFB::new(
            "fbi",
            width,
            height,
            WindowOptions {
                borderless: true,
                ..WindowOptions::default()
            },
        )?;
    }
}
