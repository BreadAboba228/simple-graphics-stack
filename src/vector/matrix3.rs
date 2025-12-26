use std::ops::{Add, Mul};

use crate::{num_traits::Consts, vector::{vec3::Vec3, AxisUnits, Unit}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Matrix3<T> {
    pub i: Vec3<T>,
    pub j: Vec3<T>,
    pub k: Vec3<T>
}

impl<T: Copy> Matrix3<T> {
    pub const fn new(i: Vec3<T>, j: Vec3<T>, k: Vec3<T>) -> Self {
        Self { i, j, k }
    }

    pub const fn row_major_new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        let i = Vec3::new(x.x, y.x, z.x);
        let j = Vec3::new(x.y, y.y, z.y);
        let k = Vec3::new(x.z, y.z, z.z);
        Self::new(i, j, k)
    }
}

impl<T: Copy + Consts> Unit for Matrix3<T> {
    const UNIT: Self = Self::new(Vec3::X, Vec3::Y, Vec3::Z);
}

impl<T: Copy + Add<Output = T>> Add for Matrix3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let i = self.i + rhs.i;
        let j = self.j + rhs.j;
        let k = self.k + rhs.k;
        Self::new(i, j, k)
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul for Matrix3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let i = self * rhs.i;
        let j = self * rhs.j;
        let k = self * rhs.k;
        Self::new(i, j, k)
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul<Vec3<T>> for Matrix3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.i * rhs.x + self.j * rhs.y + self.k * rhs.z
    }
}
