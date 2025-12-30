use std::ops::{Add, Mul, Neg};

use crate::{matrix::Unit, num_traits::{NegOne, One, SinCos, Zero}, vector::{AxisUnits, vec2::Vec2}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Matrix2<T> {
    pub i: Vec2<T>,
    pub j: Vec2<T>
}

impl<T: Copy> Matrix2<T> {
    pub const fn new(i: Vec2<T>, j: Vec2<T>) -> Self {
        Self { i, j }
    }

    pub const fn row_major_new(x: Vec2<T>, y: Vec2<T>) -> Self {
        let i = Vec2::new(x.x, y.x);
        let j = Vec2::new(x.y, y.y);
        Self::new(i, j)
    }
}

impl<T: Copy + SinCos + NegOne + Neg<Output = T>> Matrix2<T> {
    pub fn rotate_matrix(rad: T) -> Self {
        let (sin, cos) = rad.sin_cos();
        let i = Vec2::new(cos, sin);
        let j = Vec2::new(-sin, cos);
        Matrix2::new(i, j)
    }
}

impl<T: Copy + Zero + One> Unit for Matrix2<T> {
    const UNIT: Self = Self::new(Vec2::X, Vec2::Y);
}

impl<T: Copy + Add<Output = T>> Add for Matrix2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let i = self.i + rhs.i;
        let j = self.j + rhs.j;
        Self::new(i, j)
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul for Matrix2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let i = self * rhs.i;
        let j = self * rhs.j;
        Self::new(i, j)
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul<Vec2<T>> for Matrix2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self.i * rhs.x + self.j * rhs.y
    }
}
