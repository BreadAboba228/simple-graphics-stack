use minifb::{Window, WindowOptions};
use simple_render_rs::render::Render;
#[cfg(test)]
use simple_render_rs::render::{app_handler::AppHandler, buffer::BufferSize};

#[cfg(test)]
struct App(BufferSize);

#[cfg(test)]
impl AppHandler for App {
    fn buffer_size(&self) -> BufferSize {
        self.0
    }

    fn redraw(&mut self, buffer: &mut simple_render_rs::render::buffer::Buffer) {
        use simple_linear_algebra_rs::vector::vec2::Vec2;
        use simple_render_rs::color::Color;

        buffer.draw_line(self.0, Vec2::new(self.0.width as isize, self.0.height as isize), Vec2::new(1, 1), Color::from_rgb(0, 255, 255));
    }
}

#[test]
fn window_test() {
    let size = BufferSize::new(1000, 1000);
    let mut app = App(size);
    let mut options = WindowOptions::default();
    options.resize = true;
    let window = Window::new("Test", size.width, size.height, options).unwrap();

    let mut render = Render::new(&mut app, 60.0, window);
    render.run();
}
