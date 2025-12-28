#[cfg(test)]
use simple_render_rs::{render::buffer::BufferSize};
#[cfg(test)]
use simple_linear_algebra_rs::{matrix::{Unit, matrix4::Matrix4}, vector::{Axis, vec3::Vec3}};
#[cfg(test)]
use simple_3d_rs::{engine::{Engine, shape::{AngleUnit, cube::Cube}}};

#[test]
fn test_run() {
    let cube = Cube::new(Vec3::new(0.0, 0.0, 10.0), 10.0);
    //let cube = Cube::new(Vec3::ZERO, 0.25);

    let buffer_size = BufferSize::new(60, 250);

    let angles = [AngleUnit::new(Axis::X, 3.0), AngleUnit::new(Axis::Z, 3.0)];

    //let matrix = Matrix4::new_perspective(90.0, 250.0 / 60.0, 0.01, 100.0);
    let matrix = Matrix4::UNIT;

    let mut engine = Engine::new(cube.create(), '#', buffer_size, &angles, matrix);
    engine.run(60.0);
}
