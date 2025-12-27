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

pub struct Render<'a, T> {
    app: &'a mut T,
    fps: f64,
}

impl<'a, T: AppHandler> Render<'a, T> {
    pub const fn new(app: &'a mut T, fps: f64) -> Self {
        Self { app, fps }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;

        let mut buffer = Buffer::new(self.app.buffer_size());

        loop {
            self.app.redraw(&mut buffer);

            buffer.display();

            buffer.clear();

            wait(tick);

            clear_console();
        }
    }
}
