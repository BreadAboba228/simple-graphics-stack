use std::ops::{Add, Div, Mul, Sub};


use crate::{num_traits::{Consts, Sqrt}, vector::{AxisUnits, quaternion::Quaternion, vec3::Vec3, vec4::Vec4}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T: Copy> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: T) -> Self {
        Self::new(v, v)
    }
}

impl <T: Copy + Consts> Vec2<T> {
    pub fn to_homogeneous(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, T::ONE)
    }
}

impl<T: Copy + Consts> Consts for Vec2<T> {
    const ZERO: Self = Self::splat(T::ZERO);
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Copy + Consts> AxisUnits for Vec2<T> {
    const X: Self = Self::new(T::ONE, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ONE);
    const Z: Self = Self::new(T::ZERO, T::ZERO);
}

impl<T: Copy + Sqrt + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Consts + PartialEq> Vec2<T> {
    pub fn to_normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();

        if len == T::ZERO {
            Self::ZERO
        } else {
            Self::new(self.x / len, self.y / len)
        }
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self::new(x, y)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self::new(x, y)
    }
}

impl<T: Copy + Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Self::new(x, y)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self::new(x, y)
    }
}

impl<T: Copy> From<Vec3<T>> for Vec2<T> {
    fn from(value: Vec3<T>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl<T: Copy> From<Vec4<T>> for Vec2<T> {
    fn from(value: Vec4<T>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl<T: Copy> From<Quaternion<T>> for Vec2<T> {
    fn from(value: Quaternion<T>) -> Self {
        Self::new(value.x, value.y)
    }
}
