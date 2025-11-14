use simple_render_rs::{render::{Render, buffer::{Buffer, BufferSize}}, vector::vec2::Vec2};

pub struct Builder {
    size: BufferSize,
    func: fn(isize) -> isize,
    ch: char,
}

impl Builder {
    pub fn new(size: BufferSize, func: fn(isize) -> isize, ch: char) -> Self {
        Self { size, func, ch }
    }

    pub fn render_frame(&self) -> Buffer {
        let mut buffer = Buffer::new(self.size);

        let mut vec2_vec = Vec::<Vec2<isize>>::with_capacity(self.size.width);

        for x in 1..self.size.width as isize {
            let y = self.size.height as isize - (self.func)(x);

            let vec2 = Vec2::new(x, y / 2);

            vec2_vec.push(vec2);
        }

        let mut iter = vec2_vec.iter().peekable();

        while let Some(vec) = iter.next() {
            if let Some(&f) = iter.peek() {
                buffer.draw_line(self.size, *vec, *f, self.ch);
            }
        }

        buffer
    }

    pub fn run(&self) {
        let func = || -> Buffer {
            self.render_frame()
        };

        let mut render = Render::new(30.0, func);

        render.run();
    }
}