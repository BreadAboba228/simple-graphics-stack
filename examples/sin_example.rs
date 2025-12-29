use graph_builder_rs::Builder;
use minifb::{Window, WindowOptions};
use simple_render_rs::{color::Color, render::buffer::BufferSize};

fn main() {
    let size = BufferSize::new(900, 1600);

    let func = |x| ((x as f64).to_radians().sin() * 400.0) as isize;

    let mut app = Builder::new(size, func, Color::from_rgb(255, 255, 255));
    let window = Window::new("sin test", size.width, size.height, WindowOptions::default()).unwrap();

    app.run(10.0, window);
}
