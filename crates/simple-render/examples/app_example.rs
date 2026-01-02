use minifb::{Window, WindowOptions};
use simple_render::{color::Color, render::{Render, app_handler::AppHandler, buffer::{Buffer, BufferSize}}};

struct App(BufferSize, Color);

impl AppHandler for App {
    fn buffer_size(&self) -> BufferSize {
        self.0
    }

    fn redraw(&mut self, buffer: &mut Buffer) {
        use simple_linear_algebra::vector::vec2::Vec2;

        buffer.fill(Color::BLACK);

        buffer.draw_line(self.0, Vec2::new(self.0.width as isize, self.0.height as isize), Vec2::new(1, 1), self.1);
    }
}

fn main() {
    let size = BufferSize::new(1000, 1000);

    let mut app = App(size, Color::from_rgb(0, 255, 255));

    let mut options = WindowOptions::default();
    options.resize = true;

    let window = Window::new("Test", size.width, size.height, options).unwrap();

    let mut render = Render::new(&mut app, 60.0, window);
    render.run();
}
