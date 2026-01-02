use std::{thread, time::Duration};

use simple_render_rs::{color::Color, render::{app_handler::AppHandler, buffer::{Buffer, BufferSize}}};

//-6% load
fn main() {
    let size = BufferSize::new(1000, 1000);

    let mut app = App(size, Color::from_rgb(0, 255, 255));

    let mut render = Render::new(&mut app, 60.0);
    render.run();
}

fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<'a, T> {
    app: &'a mut T,
    fps: f64
}

impl<'a, T: AppHandler + Send> Render<'a, T> {
    pub const fn new(app: &'a mut T, fps: f64) -> Self {
        Self { app, fps }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;
        let size = self.app.buffer_size();


        let mut front = Buffer::new(size);
        let mut back = Buffer::new(size);

        self.app.redraw(&mut front);

        loop {
            thread::scope(|s| {
                s.spawn(|| {
                    self.app.redraw(&mut back);
                });

                wait(tick);
            });

            std::mem::swap(&mut front, &mut back);
        }


    }
}

struct App(BufferSize, Color);

impl AppHandler for App {
    fn buffer_size(&self) -> BufferSize {
        self.0
    }

    fn redraw(&mut self, buffer: &mut Buffer) {
        use simple_linear_algebra_rs::vector::vec2::Vec2;
        use simple_render_rs::color::Color;

        //black
        buffer.fill(Color::new(0));

        buffer.draw_line(self.0, Vec2::new(self.0.width as isize, self.0.height as isize), Vec2::new(1, 1), self.1);
    }
}
