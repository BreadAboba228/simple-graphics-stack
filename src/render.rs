use std::{thread, time::Duration};

use crate::render::{app_handler::AppHandler, buffer::Buffer};

pub mod buffer;
pub mod app_handler;

fn clear_console() {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("OS error");
    }
    #[cfg(not(target_os = "windows"))]
    {
        print!("\x1B[2J\x1B[H");
    }
}

fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<T> {
    app: T,
    fps: f64,
}

impl<T: AppHandler + Send> Render<T> {
    pub const fn new(app: T, fps: f64) -> Self {
        Self { app, fps }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;
        let size = self.app.buffer_size();


        let mut front = Buffer::new(size);
        let mut back = Buffer::new(size);

        loop {
            clear_console();

            thread::scope(|s| {
                s.spawn(|| {
                    self.app.redraw(&mut back);
                });

                front.display();

                front.clear();

                wait(tick);
            });

            std::mem::swap(&mut front, &mut back);
        }
    }
}
