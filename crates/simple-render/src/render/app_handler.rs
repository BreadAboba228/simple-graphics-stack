use minifb::Key;

use crate::render::buffer::{Buffer, BufferSize};

pub trait AppHandler {
    fn event(&mut self, event: Event);

    fn need_to_redraw(&self) -> bool;
}

pub enum Event<'a> {
    KeyPressed { key: Key },
    RedrawReqiest { buffer: &'a mut Buffer, size: BufferSize }
}
