use std::ops::{Add, Div, Mul, Sub};

use crate::{num_traits::{Consts, NegOne, Sqrt}, vector::{AxisUnits, quaternion::Quaternion, vec2::Vec2, vec4::Vec4}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Copy> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: T) -> Self {
        Self::new(v, v, v)
    }
}
impl<T: Copy + Consts> Vec3<T> {
    pub fn to_homogeneous(&self) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, T::ONE)
    }
}

impl<T: Copy + Div<Output = T> + Consts + PartialEq> Vec3<T> {
    pub fn to_affine(&self) -> Vec2<T> {
        if self.z == T::ZERO {
            Vec2::splat(T::ZERO)
        } else {
            let x = self.x / self.z;
            let y = self.y / self.z;
            Vec2::new(x, y)
        }
    }
}

impl<T: Copy + Sqrt + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Consts + PartialEq> Vec3<T> {
    pub fn to_normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        if len == T::ZERO {
            Self::ZERO
        } else {
           Self::new(self.x / len, self.y / len, self.z / len)
        }
    }
}

impl<T: Copy + NegOne + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Consts> Vec3<T> {
    pub fn raw_rotate(&self, quaternion: Quaternion<T>) -> Self {
        let conjugated = quaternion.to_conjugated();
        (quaternion * (*self).into() * conjugated).into()
    }
}

impl<T: Copy + Consts> Consts for Vec3<T> {
    const ZERO: Self = Self::splat(T::ZERO);
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Copy + Consts> AxisUnits for Vec3<T> {
    const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self::new(x, y, z)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self::new(x, y, z)
    }
}

impl<T: Copy + Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self::new(x, y, z)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self::new(x, y, z)
    }
}

impl<T: Copy + Consts> From<Vec2<T>> for Vec3<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::new(value.x, value.y, T::ZERO)
    }
}

impl<T: Copy> From<Vec4<T>> for Vec3<T> {
    fn from(value: Vec4<T>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl<T: Copy> From<Quaternion<T>> for Vec3<T> {
    fn from(value: Quaternion<T>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}
