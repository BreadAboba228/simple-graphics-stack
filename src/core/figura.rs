use simple_render_rs::vector::{Axis, Unit, quaternion::Quaternion, vec3::Vec3};

#[derive(Clone)]
pub struct EdgeUnit(pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct AngleUnit(pub Axis, pub f64);

impl AngleUnit {
    pub fn new(axis: Axis, degrees: f64) -> Self {
        Self(axis, degrees)
    }
    
    pub fn to_raw_quater(&self) -> Quaternion<f64> {
        let rad = self.1.to_radians();
        let axis = self.0.to_vec();
        Quaternion::rotator(rad, axis)
    }
}

#[derive(Clone)]
pub struct Figura {
    vertexes: Vec<Vec3<f64>>,
    edges: Vec<EdgeUnit>,
    center: Vec3<f64>
}

impl Figura {
    pub const fn new(
        vertexes: Vec<Vec3<f64>>,
        edges:  Vec<EdgeUnit>,
        center: Vec3<f64>
    ) -> Self {
        Self { vertexes, edges, center }
    }

    pub fn new_cube(center: Vec3<f64>, edge_len: f64) -> Self {
        let hedge = edge_len / 2.0;

        let edges: Vec<EdgeUnit> = vec![
            EdgeUnit(0, 1), EdgeUnit(0, 2), EdgeUnit(0, 4),
            EdgeUnit(1, 3), EdgeUnit(1, 5), EdgeUnit(2, 3),
            EdgeUnit(2, 6), EdgeUnit(3, 7), EdgeUnit(4, 5),
            EdgeUnit(4, 6), EdgeUnit(5, 7), EdgeUnit(6, 7)
        ];

        let x_1 = center.x - hedge;
        let x_2 = center.x + hedge;
        let y_1 = center.y - hedge;
        let y_2 = center.y + hedge;
        let z_1 = center.z - hedge;
        let z_2 = center.z + hedge;

        let vertexes = vec![
            Vec3::new(x_1, y_1, z_1), Vec3::new(x_1, y_1, z_2),
            Vec3::new(x_1, y_2, z_1), Vec3::new(x_1, y_2, z_2),
            Vec3::new(x_2, y_1, z_1), Vec3::new(x_2, y_1, z_2),
            Vec3::new(x_2, y_2, z_1), Vec3::new(x_2, y_2, z_2)
        ];

        Figura::new(vertexes, edges, center)
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

    pub fn rotate(&mut self, angles: &[AngleUnit]) {
        let mut q = Quaternion::UNIT;

        for &unit in angles {
            q = q * unit.to_raw_quater();
        }

        q.normalize();
        
        for i in &mut self.vertexes {
            *i = i.raw_rotate(q)
        }
    }
}