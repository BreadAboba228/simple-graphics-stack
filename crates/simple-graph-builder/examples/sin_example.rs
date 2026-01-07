use simple_graph_builder::Builder;
use minifb::{Window, WindowOptions};
use simple_render::{color::Color, render::buffer::BufferSize};

fn main() {
    let size = BufferSize::new(1000, 1000);

    let func = |x| ((x as f64).to_radians().sin() * 400.0) as isize;

    let app = Builder::new(func, Color::from_rgb(255, 255, 255));
    let mut options = WindowOptions::default();
    options.resize = true;
    let window = Window::new("sin test", size.width, size.height, options).unwrap();

    app.run(10.0, window);
}
