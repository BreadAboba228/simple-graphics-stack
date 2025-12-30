use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{matrix::Unit, num_traits::{One, SinCos, Sqrt, Two, Zero}, vector::{AxisUnits, Vector, vec2::*, vec3::*, vec4::*}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Quaternion<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Quaternion<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(v: T) -> Self {
        Self::new(v, v, v, v)
    }

    pub const fn set_x(&self, x: T) -> Self {
        Self::new(x, self.y, self.z, self.w)
    }

    pub const fn set_y(&self, y: T) -> Self {
        Self::new(self.x, y, self.z, self.w)
    }

    pub const fn set_z(&self, z: T) -> Self {
        Self::new(self.x, self.y, z, self.w)
    }

    pub const fn set_w(&self, w: T) -> Self {
        Self::new(self.x, self.y, self.z, w)
    }

    pub const fn set_mut_x(&mut self, x: T) {
        self.x = x;
    }

    pub const fn set_mut_y(&mut self, y: T) {
        self.y = y;
    }

    pub const fn set_mut_z(&mut self, z: T) {
        self.z = z;
    }

    pub const fn set_mut_w(&mut self, w: T) {
        self.w = w;
    }
}

impl<T: Copy
    + Zero
    + One
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + PartialEq
    + Sqrt
> Vector for Quaternion<T> {
    fn nullify(&mut self) {
        (self.x, self.y, self.z, self.w) =
            (T::ZERO, T::ZERO, T::ZERO, T::ZERO);
    }

    fn normalize(&mut self) {
        let len = (
            self.x * self.x +
            self.y * self.y +
            self.z * self.z +
            self.w * self.w
        ).sqrt();

        if len == T::ZERO {
            self.nullify();
        } else {
            (self.x, self.y, self.z, self.w) =
                (self.x / len, self.y / len, self.z / len, self.w / len);
        }
    }

    fn to_normalized(&self) -> Self {
        let len = (
            self.x * self.x +
            self.y * self.y +
            self.z * self.z +
            self.w * self.w
        ).sqrt();

        if len == T::ZERO {
            Self::ZERO
        } else {
            Self::new(self.x / len, self.y / len, self.z / len, self.w / len)
        }
    }

    fn project(&mut self) {
        if self.w == T::ZERO {
            self.nullify();
        } else {
            let x = self.x / self.w;
            let y = self.y / self.w;
            let z = self.z / self.w;
            (self.x, self.y, self.z, self.w) = (x, y, z, T::ONE);
        }
    }

    fn to_projected(&self) -> Self {
        if self.z == T::ZERO {
            Self::ZERO
        } else {
            let x = self.x / self.w;
            let y = self.y / self.w;
            let z = self.z / self.w;

            Self::new(x, y, z, T::ONE)
        }
    }
}

impl<T: Copy + Neg<Output = T>> Quaternion<T> {
    pub fn to_conjugated(&self) -> Self {
        (-self.into_vec3()).extend_to_quater(self.w)
    }
}

impl<T: Copy + SinCos + Two + Div<Output = T> + Mul<Output = T>> Quaternion<T> {
    pub fn from_angle(rad: T, axis: Vec3<T>) -> Self {
        let (sin, cos) = (rad / T::TWO).sin_cos();
        (axis * sin).extend_to_quater(cos)
    }
}

impl<T: Copy + One + Zero> Unit for Quaternion<T> {
    const UNIT: Self = Vec3::ZERO.extend_to_quater(T::ONE);
}

impl<T: Copy + Zero> Zero for Quaternion<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Copy + One> One for Quaternion<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Copy + Zero + One> AxisUnits for Quaternion<T> {
    const X: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    const Z: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Neg<Output = T>> Neg for Quaternion<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T: Copy + Add<Output = T>> Add for Quaternion<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;

        Self::new(x, y, z, w)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Quaternion<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;

        Self::new(x, y, z, w)
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

        Quaternion::new(x, y, z, w)
    }
}

impl<T: Copy + Zero> From<Vec2<T>> for Quaternion<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::new(value.x, value.y, T::ZERO, T::ZERO)
    }
}

impl<T: Copy + Zero> From<Vec3<T>> for Quaternion<T> {
    fn from(value: Vec3<T>) -> Self {
        Self::new(value.x, value.y, value.z, T::ZERO)
    }
}

impl<T: Copy> From<Vec4<T>> for Quaternion<T> {
    fn from(value: Vec4<T>) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl<T: Copy> Quaternion<T> {
    pub fn into_vec4(&self) -> Vec4<T> {
        (*self).into()
    }

    pub fn into_vec2(&self) -> Vec2<T> {
        (*self).into()
    }

    pub fn into_vec3(&self) -> Vec3<T> {
        (*self).into()
    }
}
