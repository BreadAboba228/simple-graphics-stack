use minifb::Window;
use simple_linear_algebra_rs::vector::vec2::Vec2;
use simple_render_rs::{color::Color, render::{Render, app_handler::AppHandler, buffer::{Buffer, BufferSize}}};

pub struct Builder {
    size: BufferSize,
    func: fn(isize) -> isize,
    color: Color,
}

impl Builder {
    pub fn new(size: BufferSize, func: fn(isize) -> isize, color: Color) -> Self {
        Self { size, func, color }
    }

    pub fn run(&mut self, fps: f64, window: Window) {
        let mut render = Render::new(self, fps, window);

        render.run();
    }
}

impl AppHandler for Builder {
    fn redraw(&mut self, buffer: &mut Buffer) {
        let mut vec2_vec = Vec::<Vec2<isize>>::with_capacity(self.size.width);

        for x in 1..self.size.width as isize {
            let y = self.size.height as isize - (self.func)(x);

            let vec2 = Vec2::new(x, y / 2);

            vec2_vec.push(vec2);
        }

        let mut iter = vec2_vec.iter().peekable();

        while let Some(vec) = iter.next() {
            if let Some(&f) = iter.peek() {
                buffer.draw_line(self.size, *vec, *f, self.color);
            }
        }
    }

    fn buffer_size(&self) -> BufferSize {
        self.size
    }
}
