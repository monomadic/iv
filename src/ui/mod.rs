pub trait UIComponent {
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct Gallery {
    thumb_cache: HashMap,
}
