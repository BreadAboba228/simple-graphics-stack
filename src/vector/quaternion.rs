use std::ops::{Add, Div, Mul, Sub};

use crate::{num_traits::{Consts, NegOne, SinCos, Sqrt, Two}, vector::{AxisUnits, Unit, vec2::Vec2, vec3::Vec3, vec4::Vec4}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Quaternion<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Copy> Quaternion<T> {
    pub const fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { w, x, y, z }
    }

    pub const fn splat(v: T) -> Self {
        Self::new(v, v, v, v)
    }

    pub const fn from_vec(w: T, v: Vec3<T>) -> Self {
        Self::new(w, v.x, v.y, v.z)
    }

    pub const fn vec(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<T: Copy + NegOne + Mul<Output = T>> Quaternion<T> {
    pub fn to_conjugated(&self) -> Self {
        Self::from_vec(self.w, self.vec() * T::NEG_ONE)
    }
}

impl<T: Copy + Sqrt + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Consts + PartialEq> Quaternion<T> {
    pub fn to_normalized(&self) -> Self {
        let len = (
            self.x * self.x +
            self.y * self.y +
            self.z * self.z +
            self.w * self.w
        ).sqrt();

        if len == T::ZERO {
            Self::ZERO
        } else {
            Self::new(self.w / len, self.y / len, self.z / len, self.x / len)
        }
    }
}

impl<T: Copy + SinCos + Two + Div<Output = T> + Mul<Output = T>> Quaternion<T> {
    pub fn rotator(rad: T, axis: Vec3<T>) -> Self {
        let (sin, cos) = (rad / T::TWO).sin_cos();
        Self::from_vec(cos, axis * sin)
    }
}

impl<T: Copy + Consts> Unit for Quaternion<T> {
    const UNIT: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Copy + Consts> Consts for Quaternion<T> {
    const ZERO: Self = Self::splat(T::ZERO);
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Copy + Consts> AxisUnits for Quaternion<T> {
    const X: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    const Z: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Add<Output = T>> Add for Quaternion<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let w = self.w + rhs.w;
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self::new(w, x, y, z)
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>> Mul for Quaternion<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let w = self.w * rhs.w
            - self.x * rhs.x
            - self.y * rhs.y
            - self.z * rhs.z;
        let x = self.w * rhs.x
            + self.x * rhs.w
            + self.y * rhs.z
            - self.z * rhs.y;
        let y = self.w * rhs.y
            - self.x * rhs.z
            + self.y * rhs.w
            + self.z * rhs.x;
        let z = self.w * rhs.z
            + self.x * rhs.y
            - self.y * rhs.x
            + self.z * rhs.w;

        Quaternion::new(w, x, y, z)
    }
}

impl<T: Copy + Consts> From<Vec2<T>> for Quaternion<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::new(T::ZERO, value.x, value.y, T::ZERO)
    }
}

impl<T: Copy + Consts> From<Vec3<T>> for Quaternion<T> {
    fn from(value: Vec3<T>) -> Self {
        Self::new(T::ZERO, value.x, value.y, value.z)
    }
}

impl<T: Copy> From<Vec4<T>> for Quaternion<T> {
    fn from(value: Vec4<T>) -> Self {
        Self::new(value.w, value.x, value.y, value.z)
    }
}
