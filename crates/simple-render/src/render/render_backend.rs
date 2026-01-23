use crate::render::buffer::{Buffer, BufferSize};

pub trait RenderBackend {
    fn get_size(&self) -> BufferSize;

    fn get_keys(&self) -> Vec<Key>;

    fn update(&mut self);

    fn update_with_buffer(&mut self, buffer: &Buffer);

    fn is_running(&self) -> bool;
}

pub enum Key {
    Space
}
