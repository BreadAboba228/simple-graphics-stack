use std::{sync::{Arc, Mutex}, thread, time::{Duration, Instant}};

use minifb::Window;

use crate::{color::Color, render::{app_handler::{AppHandler, Event}, buffer::{Buffer, BufferSize}}};

pub mod buffer;
pub mod app_handler;
pub mod image;

pub fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Mouse {
    pub pos: Option<(f32, f32)>,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
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

        let size = BufferSize::from_get_size(self.window.get_size());

        let mut front = (Buffer::init(size), true);
        let mut back = (Buffer::init(size), true);

        self.app.lock().unwrap()
            .event(Event::RedrawReqiest { buffer: &mut front.0 } );

        let tick = Duration::from_secs_f64(1.0 / self.fps);
        let mut last_draw = Instant::now();

        while self.window.is_open() {
            let keys = self.window.get_keys();
            let curr_size = BufferSize::from_get_size(self.window.get_size());

            let mouse = Mouse {
                pos: self.window.get_mouse_pos(minifb::MouseMode::Discard),
                left: self.window.get_mouse_down(minifb::MouseButton::Left),
                middle: self.window.get_mouse_down(minifb::MouseButton::Middle),
                right: self.window.get_mouse_down(minifb::MouseButton::Right),
            };

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
                    self.app.lock().unwrap()
                        .event(Event::MouseRequest { mouse });

                    self.app.lock().unwrap()
                        .event(Event::KeyPressed { keys });

                    let need_to_redraw = self.app.lock().unwrap().need_to_redraw();

                    if need_to_redraw {
                        self.app.lock().unwrap().event(Event::Redrawed);
                    }

                    back.1 = is_resized || need_to_redraw;

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
                let dur = last_draw.elapsed();
                if tick > dur {
                    thread::sleep(tick - dur);
                }
                last_draw = Instant::now();
            });

            std::mem::swap(&mut front, &mut back);
        }
    }
}
