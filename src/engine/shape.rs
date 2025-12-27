use simple_render_rs::vector::{Axis, Unit, quaternion::Quaternion, vec3::Vec3};

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
        Quaternion::rotator(rad, axis)
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

    pub fn center(&self) -> &Vec3<f64> {
        &self.center
    }

    pub fn raw_rotate(&mut self, quaternion: Quaternion<f64>) {
        for i in &mut self.vertexes {
            *i = (*i - self.center).raw_rotate(quaternion) + self.center
        }
    }

    pub fn rotate(&mut self, angles: &[AngleUnit]) {

        let mut q = AngleUnit::unification_to_quater(angles);

        q = q.to_normalized();

        self.raw_rotate(q);
    }
}
