use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{matrix::matrix4::*, num_traits::{One, Sqrt, Zero}, vector::{AxisUnits, Vector, quaternion::*, vec2::*, vec4::*}};

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

    pub const fn set_x(&self, x: T) -> Self {
        Self::new(x, self.y, self.z)
    }

    pub const fn set_y(&self, y: T) -> Self {
        Self::new(self.x, y, self.z)
    }

    pub const fn set_z(&self, z: T) -> Self {
        Self::new(self.x, self.y, z)
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
> Vector for Vec3<T> {
    fn nullify(&mut self) {
        (self.x, self.y, self.z) = (T::ZERO, T::ZERO, T::ZERO);
    }

    fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        if len == T::ZERO {
            self.nullify();
        } else {
            (self.x, self.y, self.z) = (self.x / len, self.y / len, self.z / len);
        }
    }

    fn to_normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        if len == T::ZERO {
            Self::ZERO
        } else {
           Self::new(self.x / len, self.y / len, self.z / len)
        }
    }

    fn project(&mut self) {
        if self.z == T::ZERO {
            self.nullify();
        } else {
            let x = self.x / self.z;
            let y = self.y / self.z;
            (self.x, self.y, self.z) = (x, y, T::ONE);
        }
    }

    fn to_projected(&self) -> Self {
        if self.z == T::ZERO {
            Vec3::ZERO
        } else {
            let x = self.x / self.z;
            let y = self.y / self.z;
            Vec3::new(x, y, T::ONE)
        }
    }
}

impl<T: Copy + One + Zero> Vec3<T> {
    pub fn into_lifted(&self) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, T::ONE)
    }

    pub fn to_displacement_matrix(&self) -> Matrix4<T> {
        let w = Vec4::new(self.x, self.y, self.z, T::ONE);

        Matrix4::new(Vec4::X, Vec4::Y, Vec4::Z, w)
    }
}

impl<T: Copy + Zero + Neg<Output = T> + Mul<Output = T> + Sub<Output = T> + Add<Output = T>> Vec3<T> {
    pub fn raw_rotate(&mut self, quater: Quaternion<T>) {
        let conjugated = quater.to_conjugated();
        *self = (quater * (*self).into_quater() * conjugated).into();
    }

    pub fn to_raw_rotated(&self, quater: Quaternion<T>) -> Self {
        let conjugated = quater.to_conjugated();
        (quater * (*self).into_quater() * conjugated).into()
    }
}

impl<T: Copy + Zero> Zero for Vec3<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Copy + One> One for Vec3<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Copy + Zero + One> AxisUnits for Vec3<T> {
    const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
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

impl<T: Copy + Zero> From<Vec2<T>> for Vec3<T> {
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

impl<T: Copy> Vec3<T> {
    pub fn into_vec2(&self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub const fn extend_to_vec4(&self, w: T) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, w)
    }

    pub const fn extend_to_quater(&self, w: T) -> Quaternion<T> {
        Quaternion::new(self.x, self.y, self.z, w)
    }
}

impl<T: Copy + Zero> Vec3<T> {
    pub fn into_vec4(&self) -> Vec4<T> {
        self.extend_to_vec4(T::ZERO)
    }

    pub fn into_quater(&self) -> Quaternion<T> {
        self.extend_to_quater(T::ZERO)
    }
}
