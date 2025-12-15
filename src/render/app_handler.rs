use crate::render::buffer::Buffer;

pub trait AppHandler {
    fn redraw(&mut self) -> Buffer;
}
