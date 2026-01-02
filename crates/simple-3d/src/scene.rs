use std::ops::Mul;

use simple_linear_algebra::{matrix::matrix4::Matrix4, vector::{Vector, quaternion::Quaternion}};

use crate::{camera::Camera, shape::{AngleUnit, Shape}};

pub struct Scene {
    shapes: Vec<Shape>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(shapes: Vec<Shape>, camera: Camera) -> Self {
        Self { shapes, camera }
    }

    pub fn raw_rotate(&mut self, quater: Quaternion<f64>) {
        for shape in &mut self.shapes {
            for vec3 in shape.mut_vertexes() {
                vec3.raw_rotate(quater);
            }
        }
    }

    pub fn raw_rotate_shapes(&mut self, quater: Quaternion<f64>) {
        for shape in &mut self.shapes {
            shape.raw_rotate(quater);
        }
    }

    pub fn rotate(&mut self, angles: &[AngleUnit]) {
        let quater = AngleUnit::unification_to_quater(angles);
        self.raw_rotate(quater);
    }

    pub fn shapes(&self) -> &[Shape] {
        &self.shapes
    }

    pub fn mut_shapes(&mut self) -> &mut [Shape] {
        &mut self.shapes
    }
}

impl Mul<Scene> for Matrix4<f64> {
    type Output = Scene;

    fn mul(self, rhs: Scene) -> Self::Output {
        let mut rhs = rhs;

        for shape in &mut rhs.shapes {
            for vec in shape.mut_vertexes() {
                *vec = (self * (*vec).into_lifted())
                    .to_projected()
                    .into_vec3();
            }
        }

        rhs
    }
}
