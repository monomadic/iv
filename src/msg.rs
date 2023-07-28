use winit::event::{ModifiersState, VirtualKeyCode};

#[derive(Debug)]
pub enum Msg {
    Init,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    KeyPress(VirtualKeyCode, ModifiersState),
}
