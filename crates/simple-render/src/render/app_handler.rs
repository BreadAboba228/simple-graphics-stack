use crate::render::{buffer::Buffer, render_backend::Key};

pub trait AppHandler {
    fn event(&mut self, event: Event);

    fn need_to_redraw(&self) -> bool;

    fn redrawed(&mut self);
}

pub enum Event<'a> {
    KeyPressed { key: Key },
    RedrawReqiest { buffer: &'a mut Buffer }
}
