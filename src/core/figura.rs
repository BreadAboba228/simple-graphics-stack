use crate::core::vector::{Axis, Quater, Vec3};

#[derive(Clone)]
pub struct EdgeUnit(pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct AngleUnit(pub Axis, pub f64);

impl AngleUnit {
    pub fn new(axis: Axis, degrees: f64) -> Self {
        Self(axis, degrees)
    }
    
    pub fn to_raw_quater(&self) -> Quater {
        let rad = self.1.to_radians();
        let axis = self.0.to_vec();
        Quater::raw_rotator(rad, axis)
    }

    pub fn to_quater(&self) -> Quater {
        self.to_raw_quater().normalize()
    }
}

#[derive(Clone)]
pub struct Figura {
    vertexes: Vec<Vec3>,
    edges: Vec<EdgeUnit>,
    center: Vec3,
}

impl Figura {
    pub const fn new(
        vertexes: Vec<Vec3>,
        edges: Vec<EdgeUnit>,
        center: Vec3
    ) -> Self {
        Self { vertexes, edges, center }
    }

    pub fn new_cube(center: Vec3, edge_len: f64) -> Self {
        let hedge = edge_len / 2.0;

        let edges = vec![
            EdgeUnit(0, 1), EdgeUnit(0, 2), EdgeUnit(0, 4),
            EdgeUnit(1, 3), EdgeUnit(1, 5), EdgeUnit(2, 3),
            EdgeUnit(2, 6), EdgeUnit(3, 7), EdgeUnit(4, 5),
            EdgeUnit(4, 6), EdgeUnit(5, 7), EdgeUnit(6, 7)
        ];

        let mut vertexes = Vec::new();

        for x in [center.x - hedge, center.x + hedge] {
            for y in [center.y - hedge, center.y + hedge] {
                for z in [center.z - hedge, center.z + hedge] {
                    vertexes.push(Vec3::new(x, y, z));
                }
            }
        }

        Figura::new(vertexes, edges, center)
    }

    pub fn edges(&self) -> &[EdgeUnit] {
        &self.edges
    }

    pub fn vertexes(&self) -> &[Vec3] {
        &self.vertexes
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn rotate(&mut self, angles: &[AngleUnit]) {
        let mut q = Quater::new(1.0, Vec3::ZERO);

        for &unit in angles {
            q = q * unit.to_raw_quater();
        }

        q.mut_normalize();
        
        for i in &mut self.vertexes {
            i.mut_raw_centered_rotate(q, self.center);
        }
    }
}