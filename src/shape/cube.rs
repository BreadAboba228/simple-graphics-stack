use simple_linear_algebra_rs::vector::vec3::Vec3;

use crate::shape::{EdgeUnit, Shape};

#[derive(Clone)]
pub struct Cube {
    center: Vec3<f64>,
    edge_len: f64
}

impl Cube {
    pub fn new(center: Vec3<f64>, edge_len: f64) -> Self {
        Self { center, edge_len }
    }

    pub fn create(&self) -> Shape {
        let half_edge = self.edge_len / 2.0;

        let x_1 = self.center.x - half_edge;
        let x_2 = self.center.x + half_edge;
        let y_1 = self.center.y - half_edge;
        let y_2 = self.center.y + half_edge;
        let z_1 = self.center.z - half_edge;
        let z_2 = self.center.z + half_edge;

        let vertexes = vec![
            Vec3::new(x_1, y_1, z_1), Vec3::new(x_1, y_1, z_2),
            Vec3::new(x_1, y_2, z_1), Vec3::new(x_1, y_2, z_2),
            Vec3::new(x_2, y_1, z_1), Vec3::new(x_2, y_1, z_2),
            Vec3::new(x_2, y_2, z_1), Vec3::new(x_2, y_2, z_2)
        ];

        let edges: Vec<EdgeUnit> = vec![
            EdgeUnit(0, 1), EdgeUnit(0, 2), EdgeUnit(0, 4),
            EdgeUnit(1, 3), EdgeUnit(1, 5), EdgeUnit(2, 3),
            EdgeUnit(2, 6), EdgeUnit(3, 7), EdgeUnit(4, 5),
            EdgeUnit(4, 6), EdgeUnit(5, 7), EdgeUnit(6, 7)
        ];

        Shape::new(vertexes, edges, self.center)
    }
}
