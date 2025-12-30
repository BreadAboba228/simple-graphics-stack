use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{matrix::{matrix2::Matrix2, matrix3::*}, num_traits::{NegOne, One, SinCos, Sqrt, Zero}, vector::{AxisUnits, Vector, quaternion::*, vec3::*, vec4::*}};

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

    pub const fn set_x(&self, x: T) -> Self {
        Self::new(x, self.y)
    }

    pub const fn set_y(&self, y: T) -> Self {
        Self::new(self.x, y)
    }

    pub const fn set_mut_x(&mut self, x: T) {
        self.x = x;
    }

    pub const fn set_mut_y(&mut self, y: T) {
        self.y = y;
    }
}

impl<T: Copy
    + One
    + Zero
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + PartialEq
    + Sqrt
> Vector for Vec2<T> {
    fn nullify(&mut self) {
        (self.x, self.y) = (T::ZERO, T::ZERO)
    }

    fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y).sqrt();

        if len == T::ZERO {
            self.nullify();
        } else {
            (self.x, self.y) = (self.x / len, self.y / len);
        }
    }

    fn to_normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();

        if len == T::ZERO {
            Vec2::ZERO
        } else {
            Vec2::new(self.x / len, self.y / len)
        }
    }

    fn project(&mut self) {
        if self.y == T::ZERO {
            self.nullify();
        } else {
            (self.x, self.y) = (self.x / self.y, T::ONE)
        }
    }

    fn to_projected(&self) -> Self {
        if self.y == T::ZERO {
            Vec2::ZERO
        } else {
            Vec2::new(self.x / self.y, T::ONE)
        }
    }
}

impl<T: Copy + One + Zero> Vec2<T> {
    pub fn into_lifted(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, T::ONE)
    }

    pub const fn into_displacement_matrix(&self) -> Matrix3<T> {
        let k = Vec3::new(self.x, self.y, T::ONE);

        Matrix3::new(Vec3::X, Vec3::Y, k)
    }
}

impl<T: Copy + SinCos + Add<Output = T> + Mul<Output = T> + Neg<Output = T> + NegOne> Vec2<T> {
    pub fn rotate(&mut self, rad: T) {
        *self = Matrix2::rotate_matrix(rad) * (*self);
    }
}

impl<T: Copy + Zero> Zero for Vec2<T> {
    const ZERO: Self = Vec2::splat(T::ZERO);
}

impl<T: Copy + One> One for Vec2<T> {
    const ONE: Self = Vec2::splat(T::ONE);
}

impl<T: Copy + Zero + One> AxisUnits for Vec2<T> {
    const X: Self = Self::new(T::ONE, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ONE);
    const Z: Self = Self::new(T::ZERO, T::ZERO);
}

impl<T: Copy + Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
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

impl<T: Copy + Zero> Vec2<T> {
    pub const fn into_vec3(&self) -> Vec3<T> {
        self.extend_to_vec3(T::ZERO)
    }

    pub const fn into_vec4(&self) -> Vec4<T> {
        self.extend_to_vec4(T::ZERO, T::ZERO)
    }

    pub const fn into_quater(&self) -> Quaternion<T> {
        self.extend_to_quater(T::ZERO, T::ZERO)
    }

    pub const fn extend_to_vec3(&self, z: T) -> Vec3<T> {
        Vec3::new(self.x, self.y, z)
    }

    pub const fn extend_to_vec4(&self, z: T, w: T) -> Vec4<T> {
        Vec4::new(self.x, self.y, z, w)
    }

    pub const fn extend_to_quater(&self, z: T, w: T) -> Quaternion<T> {
        Quaternion::new(self.x, self.y, z, w)
    }
}
