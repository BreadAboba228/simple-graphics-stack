use std::{thread, time::Duration};

use minifb::Window;

use crate::{color::Color, render::{app_handler::AppHandler, buffer::Buffer}};

pub mod buffer;
pub mod app_handler;


fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<'a, T> {
    app: &'a mut T,
    fps: f64,
    window: Window,
}

impl<'a, T: AppHandler + Send> Render<'a, T> {
    pub const fn new(app: &'a mut T, fps: f64, window: Window) -> Self {
        Self { app, fps, window }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;
        let size = self.app.buffer_size();


        let mut front = Buffer::new(size);
        let mut back = Buffer::new(size);

        self.app.redraw(&mut front);

        while self.window.is_open() {
            thread::scope(|s| {
                s.spawn(|| {
                    self.app.redraw(&mut back);
                });

                self.window.update_with_buffer(&front.0, size.width, size.height).unwrap();

                front.fill(Color::new(0));

                wait(tick);
            });

            std::mem::swap(&mut front, &mut back);
        }


    }
}
