use simple_linear_algebra::{matrix::matrix4::Matrix4, vector::vec2::Vec2};
use simple_render::render::buffer::BufferSize;

use crate::shape::Shape;

pub struct RenderCache {
    pool: Vec<Vec<Vec2<isize>>>,
    matrix: (Matrix4<f64>, BufferSize)
}

impl RenderCache {
    fn new(pool: Vec<Vec<Vec2<isize>>>, matrix: Matrix4<f64>, size: BufferSize) -> Self {
        RenderCache { pool, matrix: (matrix, size) }
    }

    pub fn init(shapes: &[Shape], size: BufferSize) -> Self {
        let mut pool = Vec::with_capacity(shapes.len());

        for shape in shapes {
            let vec = Vec::with_capacity(shape.vertexes().len());

            pool.push(vec);
        }

        let matrix = Matrix4::new_perspective(90.0, size.width as f64 / size.height as f64, 0.1, 100.0);

        Self::new(pool, matrix, size)
    }

    pub fn push(&mut self, index: usize, value: Vec2<isize>) {
        self.pool[index].push(value);
    }

    pub fn get(&self, index1: usize, index2: usize) -> Vec2<isize> {
        self.pool[index1][index2]
    }

    pub fn clear(&mut self) {
        for vec in &mut self.pool {
            vec.clear();
        }
    }

    pub fn matrix(&self) -> Matrix4<f64> {
        self.matrix.0
    }

    pub fn matrix_size(&self) -> BufferSize {
        self.matrix.1
    }

    pub fn reload_matrix(&mut self, new_size: BufferSize) {
        let matrix = Matrix4::new_perspective(90.0, new_size.width as f64 / new_size.height as f64, 0.1, 100.0);

        self.matrix = (matrix, new_size);
    }
}
