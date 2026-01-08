use std::{sync::{Arc, Mutex}, thread, time::Duration};

use minifb::Window;

use crate::{color::Color, render::{app_handler::{AppHandler, Event}, buffer::{Buffer, BufferSize}}};

pub mod buffer;
pub mod app_handler;
pub mod image;

pub fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Render<T> {
    app: Arc<Mutex<T>>,
    fps: f64,
    window: Window,
}

impl<'a, T: AppHandler + Send + Sync> Render<T> {
    pub const fn new(app: Arc<Mutex<T>>, fps: f64, window: Window) -> Self {
        Self { app, fps, window }
    }

    pub fn run(&mut self) {
        let tick = 1.0 / self.fps;

        let size = BufferSize::from_get_size(self.window.get_size());


        let mut front = (Buffer::init(size), true);
        let mut back = (Buffer::init(size), true);

        self.app.lock().unwrap()
            .event(Event::RedrawReqiest { buffer: &mut front.0 } );

        while self.window.is_open() {
            let keys = self.window.get_keys();
            let r_size = BufferSize::from_get_size(self.window.get_size());

            thread::scope(|s| {
                s.spawn(|| {
                    let is_resized = back.0.size != r_size;

                    if is_resized {
                        let target_len = r_size.width * r_size.height;

                        if target_len > back.0.raw_buffer.0.len() {
                            back.0.raw_buffer.0.resize(target_len, Color::BLACK.0);
                        }

                        back.0.size = r_size;
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
                    self.window.update_with_buffer(&front.0.raw_buffer.0, front.0.size.width, front.0.size.height).unwrap();
                } else {
                    self.window.update();
                }

                wait(tick);
            });

            std::mem::swap(&mut front, &mut back);
        }
    }
}
