use simple_linear_algebra::vector::vec3::Vec3;
use simple_render::color::Color;

use crate::shape::Shape;

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

        let triangles = vec![
            (Vec3 { x: 0, y: 2, z: 6 }, Color::BLUE), (Vec3 { x: 0, y: 6, z: 4 }, Color::BLUE),
            (Vec3 { x: 2, y: 3, z: 7 }, Color::RED), (Vec3 { x: 2, y: 7, z: 6 }, Color::RED),
            (Vec3 { x: 4, y: 6, z: 7 }, Color::GREEN), (Vec3 { x: 4, y: 7, z: 5 }, Color::GREEN),

            (Vec3 { x: 3, y: 5, z: 7 }, Color::BLUE), (Vec3 { x: 3, y: 1, z: 5 }, Color::BLUE),
            (Vec3 { x: 1, y: 0, z: 4 }, Color::RED), (Vec3 { x: 1, y: 4, z: 5 }, Color::RED),
            (Vec3 { x: 2, y: 0, z: 1 }, Color::GREEN), (Vec3 { x: 2, y: 1, z: 3 }, Color::GREEN),
        ];

        Shape::new(vertexes, triangles, self.center)
    }
}
