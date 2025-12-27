use simple_render_rs::{render::{Render, app_handler::AppHandler, buffer::{Buffer, BufferSize}}, vector::{matrix4::Matrix4, quaternion::Quaternion, vec2::Vec2}};

use crate::engine::shape::{AngleUnit, Shape};

pub mod shape;

pub struct Engine {
    shape: Shape,
    ch: char,
    buffer_size: BufferSize,
    quater: Quaternion<f64>,
    matrix: Matrix4<f64>
}

impl Engine {
    pub fn new(
        shape: Shape,
        ch: char,
        buffer_size: BufferSize,
        angles: &[AngleUnit],
        matrix: Matrix4<f64>
    ) -> Self {
        let quater = AngleUnit::unification_to_quater(angles).to_normalized();
        Self { shape, ch, buffer_size, quater, matrix }
    }

    pub fn run(&mut self, fps: f64) {
        let mut render = Render::new(self, fps);

        render.run();
    }

    pub fn to_real(&self, vec: Vec2<f64>) -> Vec2<isize> {
        let x_ratio = self.buffer_size.height as f64 / self.buffer_size.width as f64;

        let x = ((vec.x * x_ratio + 1.0) / 2.0) * self.buffer_size.width as f64;
        // dividing y by 2 because of the character size
        let y =  (1.0 - (vec.y / 2.0 + 1.0) / 2.0) * self.buffer_size.height as f64;

        Vec2::new(x as isize, y as isize)
    }
}

impl AppHandler for Engine {
    fn redraw(&mut self, buffer: &mut Buffer) {

        for edge in self.shape.edges() {
            let point1 = (self.matrix * self.shape.vertexes()[edge.0].to_homogeneous()).to_affine();
            let point2 = (self.matrix * self.shape.vertexes()[edge.1].to_homogeneous()).to_affine();

            let start = self.to_real(point1.to_affine());

            let end = self.to_real(point2.to_affine());

            //TODO: replace isize with usize in draw_line
            buffer.draw_line(self.buffer_size, start, end, self.ch);
        }

        self.shape.raw_rotate(self.quater);
    }


    fn buffer_size(&self) -> BufferSize {
        self.buffer_size
    }
}
