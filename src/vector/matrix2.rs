use std::ops::{Add, Mul};

use crate::{num_traits::Consts, vector::{vec2::Vec2, AxisUnits, Unit}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Matrix2<T> {
    pub i: Vec2<T>,
    pub j: Vec2<T>
}

impl<T: Copy> Matrix2<T> {
    pub const fn new(i: Vec2<T>, j: Vec2<T>) -> Self {
        Self { i, j }
    }
}

impl<T: Copy + Consts> Unit for Matrix2<T> {
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