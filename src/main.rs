mod core;

use core::{figura::Figura, vector::Vec3, Engine};

use crate::core::{figura::AngleUnit, vector::Axis, BufferSize};

fn main() {
    let figura = Figura::new_cube(Vec3::ZERO, 2.0);

    let buffer_size = BufferSize::new(24, 80);

    let angles = [AngleUnit::new(Axis::X, 3.0), AngleUnit::new(Axis::Z, 3.0)];

    let mut engine = Engine::new(figura, '#', 60.0, buffer_size, &angles);

    engine.run();
}