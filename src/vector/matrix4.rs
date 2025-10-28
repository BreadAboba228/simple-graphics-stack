use std::ops::{Add, Mul};

use crate::{num_traits::Consts, vector::{AxisUnits, Unit, vec3::Vec3, vec4::Vec4}};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Matrix4<T> {
    pub i: Vec4<T>,
    pub j: Vec4<T>,
    pub k: Vec4<T>,
    pub w: Vec4<T>
}

impl<T: Copy> Matrix4<T> {
    pub const fn new(i: Vec4<T>, j: Vec4<T>, k: Vec4<T>, w: Vec4<T>) -> Self {
        Self { i, j, k, w }
    }
}

impl<T: Copy + Consts> Matrix4<T> {
    pub const fn offset(shift: Vec3<T>) -> Self {
        Self::new(Vec4::X, Vec4::Y, Vec4::Z, Vec4::new(shift.x, shift.y, shift.z, T::ONE))
    }
}

impl<T: Copy + Consts> Unit for Matrix4<T> {
    const UNIT: Self = Self::new(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);
}

impl<T: Copy + Add<Output = T>> Add for Matrix4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let i = self.i + rhs.i;
        let j = self.j + rhs.j;
        let k = self.k + rhs.k;
        let w = self.w + rhs.w;
        Self::new(i, j, k, w)
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul for Matrix4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let i = self * rhs.i;
        let j = self * rhs.j;
        let k = self * rhs.k;
        let w = self * rhs.w;
        Self::new(i, j, k, w)
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul<Vec4<T>> for Matrix4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        self.i * rhs.x + self.j * rhs.y + self.k * rhs.z + self.w * rhs.w
    }
}