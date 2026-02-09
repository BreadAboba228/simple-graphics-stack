use minifb::Key;

use crate::render::{Mouse, buffer::Buffer};

pub trait AppHandler {
    fn event(&mut self, event: Event);

    fn need_to_redraw(&self) -> bool;
}

pub enum Event<'a> {
    KeyPressed { keys: Vec<Key>, mouse: Mouse },
    RedrawReqiest { buffer: &'a mut Buffer },
    Redrawed,
}
