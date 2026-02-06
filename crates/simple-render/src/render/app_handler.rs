use minifb::{Key, MouseButton};

use crate::render::buffer::Buffer;

pub trait AppHandler {
    fn event(&mut self, event: Event);

    fn need_to_redraw(&self) -> bool;
}

pub enum Event<'a> {
    KeyPressed { keys: Vec<Key> },
    MousePressed { button: MouseButton, pressed: bool },
    RedrawReqiest { buffer: &'a mut Buffer },
    MousePos { pos: (f32, f32) },
    Redrawed,
}
