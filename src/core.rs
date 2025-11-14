use simple_render_rs::{render::{Render, buffer::{Buffer, BufferSize}}, vector::{matrix4::Matrix4, vec2::Vec2, vec3::Vec3, vec4::Vec4}};

use crate::core::figura::{AngleUnit, Figura};

pub mod figura;

pub struct Engine<'a> {
    figura: Figura, 
    symbol: char,
    fps: f64,
    buffer_size: BufferSize,
    angles: &'a[AngleUnit],
    matrix: Matrix4<f64>
}

impl<'a> Engine<'a> {
    pub const fn new(
        figura: Figura, 
        symbol: char, 
        fps: f64, 
        buffer_size: BufferSize,
        angles: &'a[AngleUnit],
        matrix: Matrix4<f64>
    ) -> Self {
        Self { figura, symbol, fps, buffer_size, angles, matrix }
    }

    pub fn from_vec3(
        vector: Vec3<f64>,
        buffer_size: BufferSize,
        scale: isize
    ) -> Vec2<isize> {
        Vec2::new(
            (vector.x * scale as f64 + (buffer_size.width / 2) as f64) as isize,
            (vector.y * scale as f64 / 2.0 + (buffer_size.height / 2) as f64 ) as isize
        )
    }

    pub fn render_frame(&self) -> Buffer {
        let mut buffer = Buffer::new(self.buffer_size);

        for edge in self.figura.edges() {
            let start = Self::from_vec3(
                (self.matrix * Into::<Vec4<f64>>::into(self.figura.vertexes()[edge.0])).into(), 
                self.buffer_size,
                10
            );

            let end = Self::from_vec3(
                (self.matrix * Into::<Vec4<f64>>::into(self.figura.vertexes()[edge.1])).into(),
                self.buffer_size,
                10
            );

            buffer.draw_line(self.buffer_size, start, end, self.symbol);
    }

    buffer
}

    pub fn run(&mut self) {
        let fps = self.fps;

        let func = || -> Buffer {
            let buffer = self.render_frame();
            self.figura.rotate(self.angles);
            buffer
        };
        
        let mut render = Render::new(fps, func);

        render.run();
    }
}