use softbuffer::GraphicsContext;

pub struct SoftBufferRenderer {
    buffer: Vec<u32>,
    ctx: GraphicsContext,
    // cache:
}

impl SoftBufferRenderer {
    fn new(width: usize, height: usize, ctx: GraphicsContext) -> Self {
        SoftBufferRenderer {
            // create screen buffer (black screen)
            buffer: vec![0; width as usize * height as usize],
            ctx,
        }
    }
}
