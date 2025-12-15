use std::{thread, time::Duration};

use crate::render::{app_handler::AppHandler};

pub mod buffer;
pub mod app_handler;

fn clear_console() {
    std::process::Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .expect("OS error");
}

fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<'a, T> {
    fps: f64,
    app: &'a mut T
}

impl<'a, T: AppHandler> Render<'a, T> {
    pub const fn new(fps: f64, app: &'a mut T) -> Self {
        Self { fps, app }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;

        loop {
            self.app.redraw().display();

            wait(tick);

            clear_console();
        }
    }
}
