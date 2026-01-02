use minifb::{Window, WindowOptions};

use simple_linear_algebra_rs::matrix::Unit;
use simple_render_rs::color::Color;

use simple_render_rs::{render::buffer::BufferSize};

use simple_linear_algebra_rs::{matrix::matrix4::Matrix4, vector::{Axis, vec3::Vec3}};

use simple_3d_rs::{engine::{Engine, shape::{AngleUnit, cube::Cube}}};

fn main() {
    let cube = Cube::new(Vec3::new(0.0, 0.0, 1.0), 0.5);

    let size = BufferSize::new(1000, 1000);

    let angles = [AngleUnit::new(Axis::X, 0.5), AngleUnit::new(Axis::Y, 0.5), AngleUnit::new(Axis::Z, 0.5)];

    //let matrix = Matrix4::new_perspective(90.0, size.width as f64 / size.height as f64, 0.01, 100.0);
    let matrix = Matrix4::UNIT;

    let mut option = WindowOptions::default();
    option.resize = true;
    let window = Window::new("Test", size.width, size.height, option).unwrap();

    let mut engine = Engine::new(cube.create(), Color::from_rgb(255, 255, 0), size, &angles, matrix);

    engine.run(120.0, window);
}
