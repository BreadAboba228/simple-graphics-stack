use simple_linear_algebra::vector::vec2::Vec2;

use crate::shape::Shape;

pub struct RenderCache {
    pool: Vec<Vec<Vec2<isize>>>
}

impl RenderCache {
    fn new(pool: Vec<Vec<Vec2<isize>>>) -> Self {
        RenderCache { pool }
    }

    pub fn init(shapes: &[Shape]) -> Self {
        let mut pool = Vec::with_capacity(shapes.len());

        for shape in shapes {
            let vec = Vec::with_capacity(shape.vertexes().len());

            pool.push(vec);
        }

        Self::new(pool)
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
}
