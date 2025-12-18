use crate::render::buffer::{Buffer, BufferSize};

pub trait AppHandler {
    fn redraw(&mut self, buffer: &mut Buffer);

    fn buffer_size(&self) -> BufferSize;
}
