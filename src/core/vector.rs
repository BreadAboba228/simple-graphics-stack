use std::ops::{Add, Mul, Sub};


#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z
}

impl Axis {
    pub const fn to_vec(&self) -> Vec3 {
        match *self {
            Axis::X => Vec3::X,
            Axis::Y => Vec3::Y,
            Axis::Z => Vec3::Z
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);

    pub const Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    pub const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn to_quater(self) -> Quater {
        Quater::from_vec(self)
    }

    pub fn from_quater(quater: Quater) -> Self {
        quater.vector
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x
            + self.y * self.y
            + self.z * self.z).sqrt();

        if len == 0.0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            Vec3::new(
                self.x / len,
                self.y / len,
                self.z / len
            )
        }
    }

    pub fn mut_normalize(&mut self) {
        let len = (self.x * self.x
            + self.y * self.y
            + self.z * self.z).sqrt();
        if len == 0.0 {
            (self.x, self.y, self.z) = (0.0, 0.0, 0.0);
        } else {
            self.x = self.x / len;
            self.y = self.y / len;
            self.z = self.z / len;
        }
    }

    pub fn raw_rotate(&self, quater: Quater) -> Self {
        let conjugated = quater.to_conjugated();
        (quater * self * conjugated).to_vec()
    }

    pub fn raw_mut_rotate(&mut self, quater: Quater) {
        Vec3 { x: self.x, y: self.y, z: self.z } = self.raw_rotate(quater);
    }

    pub fn rotate(&self, angle: f64, axis: Vec3) -> Self {
        let rad = angle.to_radians();
        let axis = axis.normalize();
        let quater = Quater::raw_rotator(rad, axis);
        self.raw_rotate(quater)
    }

    pub fn mut_rotate(&mut self, angle: f64, axis: Vec3) {
        let rad = angle.to_radians();
        let axis = axis.normalize();
        let quater = Quater::raw_rotator(rad, axis);
        self.raw_mut_rotate(quater);
    }

    pub fn centered_rotate(&self, angle: f64, axis: Vec3, center: Vec3) -> Self {
        (*self - center).rotate(angle, axis) + center
    }

    pub fn mut_centered_rotate(&mut self, angle: f64, axis: Vec3, center: Vec3) {
        Vec3 { x: self.x, y: self.y, z: self.z}
        = self.centered_rotate(angle, axis, center);
    }

    pub fn raw_centered_rotate(&self, quater: Quater, center: Vec3) -> Self {
        (*self - center).raw_rotate(quater) + center
    }

    pub fn mut_raw_centered_rotate(&mut self, quater: Quater, center: Vec3) {
        Vec3 { x: self.x, y: self.y, z: self.z}
        = self.raw_centered_rotate(quater, center);
    }
}

impl Add<Self> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vec3::new(x, y, z)
    }
}

impl Add<Quater> for Vec3 {
    type Output = Quater;

    fn add(self, rhs: Quater) -> Self::Output {
        rhs + self
    }
}

impl Sub<Self> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl Mul<Self> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Vec3::new(x, y, z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vec3::new(x, y, z)
    }
}

impl Mul<Quater> for Vec3 {
    type Output = Quater;

    fn mul(self, rhs: Quater) -> Self::Output {
        let quat = self.to_quater();
        quat * rhs
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Quater {
    pub scalar: f64,
    pub vector: Vec3
}

impl Quater {
    pub const UNIT: Quater = Quater::new(1.0, Vec3::ZERO);

    pub const fn new(scalar: f64, vector: Vec3) -> Self {
        Self { scalar, vector }
    }

    pub fn from_vec(vector: Vec3) -> Self {
        Quater::new(0.0, vector)
    }
    pub fn to_vec(&self) -> Vec3 {
        self.vector
    }

    pub fn raw_rotator(rad: f64, axis: Vec3) -> Self {
        let (sin, cos) = (rad / 2.0).sin_cos();
        Quater::new(cos, axis * sin)
    }

    pub fn rotator(rad: f64, axis: Vec3) -> Self {
        Quater::raw_rotator(rad, axis).normalize_all()
    }

    pub fn to_conjugated(&self) -> Self {
        Quater::new(self.scalar, self.vector * -1.0)
    }

    pub fn normalize(&self) -> Self {
        let len = (self.scalar * self.scalar
            + self.x() * self.x()
            + self.y() * self.y()
            + self.z() * self.z()).sqrt();

        if len == 0.0 {
            Quater::new(0.0, Vec3::new(0.0, 0.0, 0.0))
        } else {
            Quater::new(
                self.scalar / len,
                Vec3::new(
                    self.x() / len,
                    self.y() / len,
                    self.z() / len
                )
            )
        }
    }

    pub fn mut_normalize(&mut self) {
        Quater {
            scalar: self.scalar,
            vector: Vec3 {
                x: self.vector.x,
                y: self.vector.y,
                z: self.vector.z
            }
        }
        = self.normalize();
    }

    pub fn normalize_vec(&self) -> Quater {
        Quater::new(self.scalar, self.vector.normalize())
    }

    pub fn mut_normalize_vec(&mut self) {
        self.vector.mut_normalize();
    }

    pub fn normalize_all(&self) -> Quater {
        Quater::new(self.scalar, self.vector.normalize()).normalize()
    }

    pub fn mut_normalize_all(&mut self) {
        self.mut_normalize_vec();
        self.mut_normalize();
    }

    pub fn x(&self) -> f64 {
        self.vector.x
    }

    pub fn y(&self) -> f64 {
        self.vector.y
    }

    pub fn z(&self) -> f64 {
        self.vector.z
    }
}

impl Add<Self> for Quater {
    type Output = Quater;

    fn add(self, rhs: Self) -> Self::Output {
        let scalar = self.scalar + rhs.scalar;
        let vector = self.vector + rhs.vector;
        Quater::new(scalar, vector)
    }
}

impl Add<Vec3> for Quater {
    type Output = Quater;

    fn add(self, rhs: Vec3) -> Self::Output {
        let vector = self.vector + rhs;
        Quater::new(self.scalar, vector)
    }
}

impl Mul<Self> for Quater {
    type Output = Quater;

    fn mul(self, rhs: Self) -> Self::Output {
        let scalar = self.scalar * rhs.scalar
            - self.x() * rhs.x()
            - self.y() * rhs.y()
            - self.z() * rhs.z();

        let x = self.scalar * rhs.x()
            + self.x() * rhs.scalar
            + self.y() * rhs.z()
            - self.z() * rhs.y();

        let y = self.scalar * rhs.y()
            - self.x() * rhs.z()
            + self.y() * rhs.scalar
            + self.z() * rhs.x();

        let z = self.scalar * rhs.z()
            + self.x() * rhs.y()
            - self.y() * rhs.x()
            + self.z() * rhs.scalar;

        let vector = Vec3::new(x, y, z);

        Quater::new(scalar, vector)
    }
}

impl Mul<Vec3> for Quater {
    type Output = Quater;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let rhs = rhs.to_quater();
        self * rhs
    }
}

impl Mul<&Vec3> for Quater {
    type Output = Quater;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let rhs = rhs.clone();
        self * rhs
    }
}

impl Mul<&mut Vec3> for Quater {
    type Output = Quater;

    fn mul(self, rhs: &mut Vec3) -> Self::Output {
        let rhs = rhs.clone();
        self * rhs
    }
}

#[test]
fn test_quaters_mul() {
    let q = Quater::new(1.0, Vec3::new(2.0, 3.0, 4.0));
    assert_eq!(q, q * Quater::UNIT);
    assert_eq!(q, Quater::UNIT * q);
    let q = Quater::new(1.0, Vec3::new(2.0, -3.0, 1.0));
    let p = Quater::new(2.0, Vec3::new(-1.0, 2.0, 3.0));
    assert_eq!(Quater::new(7.0, Vec3::new(-8.0, -11.0, 6.0)), q * p);
}