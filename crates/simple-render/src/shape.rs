use bytemuck::{NoUninit, Pod, Zeroable};

pub struct Shape<V: Pod + Zeroable, T: NoUninit> {
    pub vertex_buf: Vec<V>,
    pub index_buf: Vec<T>,
}
