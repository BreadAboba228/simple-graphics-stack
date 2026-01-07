use std::ops::{Add, Mul, Sub};

use crate::num_traits::{One, Zero};

pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod quaternion;

pub trait Vector: Add<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Sized
    + Zero
    + One
    + AxisUnits
{
    fn nullify(&mut self);

    fn normalize(&mut self);

    fn to_normalized(&self) -> Self;

    fn project(&mut self);

    fn to_projected(&self) -> Self;
}

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

#[macro_export]
macro_rules! impl_assign {
    ($t:ty) => {
        use std::ops::{AddAssign, SubAssign, MulAssign};

        impl<T: Copy + Add<Output = T>> AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<T: Copy + Sub<Output = T>> SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<T: Copy + Mul<Output = T>> MulAssign for $t {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    };
}
