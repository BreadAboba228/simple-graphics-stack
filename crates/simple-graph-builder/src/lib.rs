use std::sync::{Arc, Mutex};

use minifb::Window;
use simple_linear_algebra::vector::vec2::Vec2;
use simple_render::{color::Color, render::{Render, app_handler::{AppHandler, Event}}};

pub struct Builder {
    func: fn(isize) -> isize,
    color: Color,
    need_to_redraw: bool
}

impl Builder {
    pub fn new(func: fn(isize) -> isize, color: Color) -> Self {
        Self { func, color, need_to_redraw: true }
    }

    pub fn run(self, fps: f64, window: Window) {

        let clone = Arc::new(Mutex::new(self));

        let mut render = Render::new(clone, fps, window);

        render.run();
    }
}

impl AppHandler for Builder {
    fn event(&mut self, event: Event) {
        match event {
            Event::RedrawReqiest { buffer, size } => {
                buffer.fill(Color::BLACK);

                let mut vec2_vec = Vec::<Vec2<isize>>::with_capacity(size.width);

                for x in 1..size.width as isize {
                    let y = size.height as isize - (self.func)(x);

                    let vec2 = Vec2::new(x, y / 2);

                    vec2_vec.push(vec2);
                }

                let mut iter = vec2_vec.iter().peekable();

                while let Some(vec) = iter.next() {
                    if let Some(&f) = iter.peek() {
                        buffer.draw_line(size, *vec, *f, self.color);
                    }
                }

                self.need_to_redraw = true;
            }

            _ => ()
        }
    }

    fn need_to_redraw(&self) -> bool {
        self.need_to_redraw
    }
}
