use simple_linear_algebra_rs::{matrix::{Unit, matrix4::Matrix4}, num_traits::Zero, vector::{quaternion::Quaternion, vec3::Vec3}};

use crate::shape::AngleUnit;

pub struct Camera {
    pub pos: Vec3<f64>,
    quaternion: Quaternion<f64>
}

impl Camera {
    pub const fn new(pos: Vec3<f64>, quaternion: Quaternion<f64>) -> Self {
        Self { pos, quaternion }
    }

    pub fn from_angles(pos: Vec3<f64>, angles: &[AngleUnit]) -> Self {
        let quaternion = AngleUnit::unification_to_quater(angles);

        Self::new(pos, quaternion)
    }

    pub fn raw_rotate(&self, quaternion: Quaternion<f64>) -> Self {
        let quaternion = self.quaternion * quaternion;

        Self::new(self.pos, quaternion)
    }

    pub fn rotate(&self, angles: &[AngleUnit]) -> Self {
        let quaternion = AngleUnit::unification_to_quater(angles);

        self.raw_rotate(quaternion)
    }

    pub fn to_displacement_matrix(&self) -> Matrix4<f64> {
        (self.pos * -1.0).to_displacement_matrix()
    }

    pub fn to_rotation_quaternion(&self) -> Quaternion<f64> {
        self.quaternion.to_conjugated()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quaternion::UNIT)
    }
}
