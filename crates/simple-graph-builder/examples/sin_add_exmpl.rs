use simple_graph_builder::Builder;
use minifb::{Window, WindowOptions};
use simple_render::{color::Color, render::buffer::BufferSize};

fn main() {
    let size = BufferSize::new(900, 1600);

    let func = |x| func1(x) + func2(x);

    let mut app = Builder::new(size, func, Color::from_rgb(255, 255, 255));
    let window = Window::new("sin test", size.width, size.height, WindowOptions::default()).unwrap();

    app.run(10.0, window);
}

fn func1(x: isize) -> isize {
    ((x as f64).to_radians().sin() * 400.0) as isize
}

fn func2(x: isize) -> isize {
    ((x as f64 * 2.0).to_radians().sin() * 300.0) as isize
}
