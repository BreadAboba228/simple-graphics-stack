use std::ops::Mul;

use simple_linear_algebra::{matrix::{Unit, matrix4::Matrix4}, vector::{Axis, Vector, quaternion::Quaternion, vec3::Vec3}};

pub mod cube;

#[derive(Clone)]
pub struct EdgeUnit(pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct AngleUnit(pub Axis, pub f64);

impl AngleUnit {
    pub fn new(axis: Axis, degrees: f64) -> Self {
        Self(axis, degrees)
    }

    pub fn to_quater(&self) -> Quaternion<f64> {
        let rad = self.1.to_radians();
        let axis = self.0.to_vec();
        Quaternion::from_angle(rad, axis)
    }

    pub fn unification_to_quater(angles: &[AngleUnit]) -> Quaternion<f64> {
        let mut q = Quaternion::UNIT;

        for &unit in angles {
            q = q * unit.to_quater();
        }

        q
    }
}

#[derive(Clone)]
pub struct Shape {
    vertexes: Vec<Vec3<f64>>,
    edges: Vec<EdgeUnit>,
    center: Vec3<f64>
}

impl Shape {
    pub const fn new(
        vertexes: Vec<Vec3<f64>>,
        edges:  Vec<EdgeUnit>,
        center: Vec3<f64>
    ) -> Self {
        Self { vertexes, edges, center }
    }

    pub fn edges(&self) -> &[EdgeUnit] {
        &self.edges
    }

    pub fn vertexes(&self) -> &[Vec3<f64>] {
        &self.vertexes
    }

    pub fn mut_vertexes(&mut self) -> &mut [Vec3<f64>] {
        &mut self.vertexes
    }

    pub fn center(&self) -> &Vec3<f64> {
        &self.center
    }

    pub fn raw_rotate(&mut self, quater: Quaternion<f64>) {
        for i in &mut self.vertexes {
            *i = (*i - self.center).to_raw_rotated(quater) + self.center;
        }
    }

    pub fn rotate(&mut self, angles: &[AngleUnit]) {

        let mut q = AngleUnit::unification_to_quater(angles);

        q.normalize();

        self.raw_rotate(q);
    }

    pub fn into_projected_vertexes(&self) -> Vec<Vec3<f64>> {
        let mut vec = self.vertexes.clone();

        for i in &mut vec {
            *i = *i + self.center;
        }

        vec
    }
}

impl Mul<Shape> for Matrix4<f64> {
    type Output = Shape;

    fn mul(self, rhs: Shape) -> Self::Output {
        let mut rhs = rhs;
        for vec in &mut rhs.vertexes {
            *vec = (self * (*vec).into_lifted())
                .to_projected()
                .into_vec3();
        }

        rhs
    }
}
