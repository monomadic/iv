#[derive(Debug)]
pub enum Msg {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Resized(u32, u32),
}
