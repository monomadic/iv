use winit::event::{ModifiersState, VirtualKeyCode};

#[derive(Debug)]
pub enum Msg {
    Init,
    KeyPress(VirtualKeyCode, ModifiersState),
}
