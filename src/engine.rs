use minifb::Window;
use simple_linear_algebra_rs::{matrix::matrix4::Matrix4, vector::{quaternion::Quaternion, vec2::Vec2}};
use simple_render_rs::{color::Color, render::{Render, app_handler::AppHandler, buffer::{Buffer, BufferSize}}};

use crate::engine::shape::{AngleUnit, Shape};

pub mod shape;

pub struct Engine {
    shape: Shape,
    color: Color,
    buffer_size: BufferSize,
    quater: Quaternion<f64>,
    matrix: Matrix4<f64>,
    vertexes_pool: Vec::<Vec2<isize>>
}

impl Engine {
    pub fn new(
        shape: Shape,
        color: Color,
        buffer_size: BufferSize,
        angles: &[AngleUnit],
        matrix: Matrix4<f64>
    ) -> Self {
        let quater = AngleUnit::unification_to_quater(angles).to_normalized();
        let vertexes_pool = Vec::with_capacity(shape.vertexes().len());
        Self { shape, color, buffer_size, quater, matrix, vertexes_pool }
    }

    pub fn run(&mut self, fps: f64, window: Window) {
        let mut render = Render::new(self, fps, window);

        render.run();
    }

    pub fn to_real(&self, vec: Vec2<f64>) -> Vec2<isize> {
        //[-1; 1] + 1 -> [0; 2]
        // [0; 2] / 2 -> [0; 1]
        // [0; 1] * width -> [0; width]
        let x = ((vec.x + 1.0) / 2.0) * self.buffer_size.width as f64;

        //[-1; 1] + 1 -> [0; 2]
        // [0; 2] / 2 -> [0; 1]
        // 1.0 - [0; 1] -> [1; 0] (axis reversal)
        // [1; 0] * height -> [height; 0]
        let y =  (1.0 - (vec.y + 1.0) / 2.0) * self.buffer_size.height as f64;

        Vec2::new(x as isize, y as isize)
    }
}

impl AppHandler for Engine {
    fn redraw(&mut self, buffer: &mut Buffer) {
        let vertexes = self.shape.vertexes();

        for i in vertexes {
            let vertex3 = (self.matrix * (*i).to_homogeneous()).to_affine();
            let vertex2 = self.to_real(vertex3.to_affine());
            self.vertexes_pool.push(vertex2);
        }

        buffer.fill(Color::new(0));

        for edge in self.shape.edges() {
            let start = self.vertexes_pool[edge.0];

            let end = self.vertexes_pool[edge.1];

            //TODO: replace isize with usize in draw_line
            buffer.draw_line(self.buffer_size, start, end, self.color);
        }

        self.shape.raw_rotate(self.quater);
        self.vertexes_pool.clear();
    }


    fn buffer_size(&self) -> BufferSize {
        self.buffer_size
    }
}
