use std::{thread, time::Duration};

use crate::render::buffer::Buffer;

pub mod buffer;

fn clear_console() {
    std::process::Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .expect("OS error");
}

fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<U> where U: FnMut() -> Buffer {
    fps: f64,
    func: U
}

impl<U> Render<U> where U: FnMut() -> Buffer {
    pub fn new(fps: f64, func: U) -> Self {
        Self { fps, func }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;
        
        loop {
            (self.func)().display();

            wait(tick);

            clear_console();
        }
    }
}