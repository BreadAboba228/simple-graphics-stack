use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crate::{color::Color, render::{app_handler::{AppHandler, Event}, buffer::Buffer, render_backend::RenderBackend}};

pub mod buffer;
pub mod app_handler;
pub mod image;
pub mod render_backend;

pub fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<T, R: RenderBackend> {
    app: Arc<Mutex<T>>,
    fps: f64,
    backend: R,
}

impl<'a, T: AppHandler + Send + Sync, R: RenderBackend> Render<T, R> {
    pub const fn new(app: Arc<Mutex<T>>, fps: f64, backend: R) -> Self {
        Self { app, fps, backend }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;

        let size = self.backend.get_size();


        let mut front = (Buffer::init(size), true);
        let mut back = (Buffer::init(size), true);

        self.app.lock().unwrap()
            .event(Event::RedrawReqiest { buffer: &mut front.0 } );

        while self.backend.is_running() {
            let keys = self.backend.get_keys();
            let curr_size = self.backend.get_size();

            thread::scope(|s| {
                s.spawn(|| {
                    let is_resized = back.0.size != curr_size;

                    if is_resized {
                        let target_len = curr_size.width * curr_size.height;

                        if target_len > back.0.raw_buffer.0.len() {
                            back.0.raw_buffer.0.resize(target_len, Color::BLACK.0);
                        }

                        back.0.size = curr_size;
                    }

                    for key in keys {
                        self.app.lock().unwrap()
                            .event(Event::KeyPressed { key });
                    }

                    back.1 = if self.app.lock().unwrap().need_to_redraw() {
                        self.app.lock().unwrap().redrawed();
                        true
                    } else {
                        is_resized
                    };

                    if back.1 {
                        self.app.lock().unwrap()
                            .event(Event::RedrawReqiest { buffer: &mut back.0 });
                    }
                });

                if front.1 {
                    self.backend.update_with_buffer(&front.0);
                } else {
                    self.backend.update();
                }

                wait(tick);
            });

            std::mem::swap(&mut front, &mut back);
        }
    }
}
