#[cfg(test)]
use simple_render_rs::{num_traits::Consts, render::buffer::BufferSize, vector::{Axis, Unit, matrix4::Matrix4, vec3::Vec3}};

use crate::core::{Engine, figura::{AngleUnit, Figura}};

#[test]
fn test_run() {
    let figura = Figura::new_cube(Vec3::ZERO, 7.5);
     
    let buffer_size = BufferSize::new(75, 250);

    let angles = [AngleUnit::new(Axis::X, 3.0), AngleUnit::new(Axis::Z, 3.0)];

    let matrix = Matrix4::UNIT;

    let mut engine = Engine::new(figura, '#', 60.0, buffer_size, &angles, matrix);

    engine.run();
}
