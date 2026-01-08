use std::{fs::File, path::Path, sync::{Arc, Mutex}, thread};

use minifb::{Key, Window, WindowOptions};
use simple_linear_algebra::vector::{vec2::Vec2, vec3::Vec3};
use simple_render::{color::Color, render::{Render, app_handler::{AppHandler, Event}, image::Image, buffer::BufferSize, wait}};

struct App(Color, bool, Image);

impl AppHandler for App {
    fn event(&mut self, event: Event) {
        match event {
            Event::KeyPressed { key } => {
                self.1 = true;
                match key {
                    Key::W => {
                        self.0.0 += 1;
                    },

                    Key::S => {
                        self.0.0 -= 1;
                    },

                    _ => (),
                }
            },

            Event::RedrawReqiest { buffer } => {
                buffer.fill(Color::BLACK);

                buffer.draw_image(&self.2, Vec2::new(0, 0));

                buffer.draw_line(Vec2::new(800, 800), Vec2::new(1, 1), self.0);

                let triangle = Vec3::new(Vec2::new(0, 0), Vec2::new(100, 400), Vec2::new(700, 200));

                buffer.fill_triangle(triangle, Color::GREEN);
            }
        }
    }

    fn need_to_redraw(&self) -> bool {
        self.1
    }

    fn redrawed(&mut self) {
        self.1 = false;
    }
}

fn main() {
    let size = BufferSize::new(1000, 1000);

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/example_image.png");

    let file = File::open(path).unwrap();

    let image = Image::from_png(file).unwrap();

    let app = App(Color::from_rgb(0, 255, 255), false, image);

    let mut options = WindowOptions::default();
    options.resize = true;

    let window = Window::new("Test", size.width, size.height, options).unwrap();

    let mut_app = Arc::new(Mutex::new(app));
    let clone = mut_app.clone();

    let mut render = Render::new(mut_app, 60.0, window);

    let _handle = thread::spawn(move || {
        loop {
            clone.lock().unwrap()
                .1 = true;

            clone.lock().unwrap()
                .0.0 += 1;

            wait(0.1);
        }
    });

    render.run();
}
