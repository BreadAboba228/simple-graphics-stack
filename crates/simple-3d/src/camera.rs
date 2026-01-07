use simple_linear_algebra::{matrix::{Unit, matrix4::Matrix4}, num_traits::Zero, vector::{Vector, quaternion::Quaternion, vec3::Vec3}};

use crate::shape::AngleUnit;

pub struct Camera {
    pub pos: Vec3<f64>,
    quater: Quaternion<f64>
}

impl Camera {
    pub const fn new(pos: Vec3<f64>, quater: Quaternion<f64>) -> Self {
        Self { pos, quater }
    }

    pub fn from_angles(pos: Vec3<f64>, angles: &[AngleUnit]) -> Self {
        let quaternion = AngleUnit::unification_to_quater(angles)
            .to_normalized();

        Self::new(pos, quaternion)
    }

    pub fn quater(&self) -> &Quaternion<f64> {
        &self.quater
    }

    pub fn raw_rotate(&mut self, quater: Quaternion<f64>) {
        self.quater = self.quater * quater;
    }

    pub fn rotate(&mut self, angles: &[AngleUnit]) {
        let quater = AngleUnit::unification_to_quater(angles);

        self.raw_rotate(quater)
    }

    pub fn to_displacement_matrix(&self) -> Matrix4<f64> {
        (-self.pos).to_displacement_matrix()
    }

    pub fn to_rotation_quaternion(&self) -> Quaternion<f64> {
        self.quater.to_conjugated()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quaternion::UNIT)
    }
}
