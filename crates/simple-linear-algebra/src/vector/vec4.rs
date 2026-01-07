use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{impl_assign, num_traits::{One, Sqrt, Zero}, vector::{AxisUnits, Vector, quaternion::*, vec2::*, vec3::*}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T: Copy> Vec4<T> {
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
> Vector for Vec4<T> {
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

impl<T: Copy + Zero + One> Vec4<T> {
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T>> Vec4<T> {
    pub fn rotate(&mut self, quater: Quaternion<T>) {
        *self = self.to_rotated(quater)
    }

    pub fn to_rotated(&self, quater: Quaternion<T>) -> Self {
        (quater * self.into_quater() * quater.to_conjugated()).into_vec4()
    }
}

impl<T: Copy + Zero> Zero for Vec4<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Copy + One> One for Vec4<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Copy + Zero + One> AxisUnits for Vec4<T> {
    const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
}

impl<T: Copy + Neg<Output = T>> Neg for Vec4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;

        Self::new(x, y, z, w)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;

        Self::new(x, y, z, w)
    }
}

impl<T: Copy + Mul<Output = T>> Mul for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * rhs.w;

        Self::new(x, y, z, w)
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

impl<T: Copy + Zero> From<Vec2<T>> for Vec4<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::new(value.x, value.y, T::ZERO, T::ZERO)
    }
}


impl<T: Copy + Zero> From<Vec3<T>> for Vec4<T> {
    fn from(value: Vec3<T>) -> Self {
        Self::new(value.x, value.y, value.z, T::ZERO)
    }
}

impl<T: Copy> From<Quaternion<T>> for Vec4<T> {
    fn from(value: Quaternion<T>) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl<T: Copy> Vec4<T> {
    pub fn into_quater(&self) -> Quaternion<T> {
        Quaternion::new(self.x, self.y, self.z, self.w)
    }

    pub fn into_vec2(&self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub fn into_vec3(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl_assign!(Vec4<T>);
