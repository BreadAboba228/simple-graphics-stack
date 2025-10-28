pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod quaternion;
pub mod matrix2;
pub mod matrix3;
pub mod matrix4;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Axis {
    X,
    Y,
    Z
}

impl Axis {
    pub fn to_vec<T: AxisUnits>(self) -> T {
        match self {
            Axis::X => T::X,
            Axis::Y => T::Y,
            Axis::Z => T::Z
        }
    }
}

pub trait AxisUnits {
    const X: Self;
    const Y: Self;
    const Z: Self;
}

pub trait Unit {
    const UNIT: Self;
}