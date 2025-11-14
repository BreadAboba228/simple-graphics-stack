use std::ops::{Add, Div, Mul};

use crate::{num_traits::Consts, vector::{AxisUnits, quaternion::Quaternion, vec2::Vec2, vec3::Vec3}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T: Copy> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4 { x, y, z, w }
    }

    pub const fn splat(v: T) -> Self {
        Vec4::new(v, v, v, v)
    }
}

impl<T: Copy + Consts> Vec4<T> {
    pub const W: Self = Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Div<Output = T>> Vec4<T> {
    pub fn to_decart_vec3(&self) -> Vec3<T> {
        Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
    }
}

impl<T: Copy + Consts> Consts for Vec4<T> {
    const ZERO: Self = Vec4::splat(T::ZERO);
    const ONE: Self = Vec4::splat(T::ONE);
}

impl<T: Copy + Consts> AxisUnits for Vec4<T> {
    const X: Self = Vec4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    const Y: Self = Vec4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    const Z: Self = Vec4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
}

impl<T: Copy + Add<Output = T>> Add for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Vec4::new(x, y, z, w)
    }
}

impl<T: Copy + Mul<Output = T>> Mul for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * self.w;
        Vec4::new(x, y, z, w)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;
        Self::new(x, y, z, w)
    }
}

impl<T: Copy + Consts> From<Vec2<T>> for Vec4<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::new(value.x, value.y, T::ZERO, T::ZERO)
    }
}


impl<T: Copy + Consts> From<Vec3<T>> for Vec4<T> {
    fn from(value: Vec3<T>) -> Self {
        Self::new(value.x, value.y, value.z, T::ZERO)
    }
}


impl<T: Copy> From<Quaternion<T>> for Vec4<T> {
    fn from(value: Quaternion<T>) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}