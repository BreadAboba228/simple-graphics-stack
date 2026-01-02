//!failed

use simple_graph_builder::Builder;
use minifb::{Window, WindowOptions};
use simple_render::{color::Color, render::buffer::BufferSize};

fn main() {
    let size = BufferSize::new(60, 320);
    let func = |x| (x as f64).to_radians().tan() as isize;
    let mut app = Builder::new(size, func, Color::from_rgb(255, 255, 255));
    let window = Window::new("sin test", size.width, size.height, WindowOptions::default()).unwrap();
    app.run(0.01, window);
}
