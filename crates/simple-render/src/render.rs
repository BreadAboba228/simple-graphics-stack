use std::{sync::{Arc, Mutex}, thread, time::Duration};

use minifb::Window;

use crate::{color::Color, render::{app_handler::{AppHandler, Event}, buffer::{Buffer, BufferSize}}};

pub mod buffer;
pub mod app_handler;


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

        let mut size = BufferSize::from_get_size(self.window.get_size());


        let mut front = (Buffer::new(size), true);
        let mut back = (Buffer::new(size), true);

        let mut is_resized;

        self.app.lock().unwrap()
            .event(Event::RedrawReqiest { buffer: &mut front.0, size } );

        while self.window.is_open() {
            let keys = self.window.get_keys();
            let r_size = BufferSize::from_get_size(self.window.get_size());

            is_resized = size != r_size;

            thread::scope(|s| {
                s.spawn(|| {
                    let target_len = r_size.width * r_size.height;

                    if target_len > back.0.0.len() {
                        back.0.0.resize(target_len, Color::BLACK.0);
                    }

                    for key in keys {
                        self.app.lock().unwrap()
                            .event(Event::KeyPressed { key });
                    }

                    back.1 = is_resized || self.app.lock().unwrap()
                        .need_to_redraw();

                    if back.1 {
                        self.app.lock().unwrap()
                            .event(Event::RedrawReqiest { buffer: &mut back.0, size: r_size });
                    }
                });

                if front.1 {
                    self.window.update_with_buffer(&front.0.0, size.width, size.height).unwrap();
                } else {
                    self.window.update();
                }

                wait(tick);
            });

            size = r_size;

            std::mem::swap(&mut front, &mut back);
        }


    }
}
