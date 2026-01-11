use simple_linear_algebra::{matrix::matrix4::Matrix4, vector::{quaternion::Quaternion, vec2::Vec2, vec3::Vec3}};
use simple_render::render::buffer::BufferSize;

use crate::{camera::Camera, shape::Shape};

pub struct RenderCache {
    pool: Vec<Vec<Vec2<isize>>>,
    persp_matrix: (Matrix4<f64>, BufferSize),
    camera: (Matrix4<f64>, Quaternion<f64>, Vec3<f64>),
}

impl RenderCache {
    fn new(pool: Vec<Vec<Vec2<isize>>>, persp_matrix: (Matrix4<f64>, BufferSize), camera: (Matrix4<f64>, Quaternion<f64>, Vec3<f64>)) -> Self {
        RenderCache { pool, persp_matrix, camera }
    }

    pub fn init(shapes: &[Shape], size: BufferSize, camera: &Camera) -> Self {
        let mut pool = Vec::with_capacity(shapes.len());

        for shape in shapes {
            let vec = Vec::with_capacity(shape.vertexes().len());

            pool.push(vec);
        }

        let persp_matrix = (Matrix4::persp_rh_matrix(90.0, size.width as f64 / size.height as f64, 0.1, 100.0), size);

        let camera = (camera.to_displacement_matrix(), camera.to_rotation_quaternion(), camera.pos);

        Self::new(pool, persp_matrix, camera)
    }

    pub fn push(&mut self, index: usize, value: Vec2<isize>) {
        self.pool[index].push(value);
    }

    pub fn get(&self, index1: usize, index2: usize) -> Vec2<isize> {
        self.pool[index1][index2]
    }

    pub fn clear(&mut self) {
        for vec in &mut self.pool {
            vec.clear();
        }
    }

    pub fn reload(&mut self, size: BufferSize, camera: &Camera) {
        if self.persp_matrix.1 != size {
            self.reload_persp_matrix(size);
        }

        if self.camera.2 != camera.pos {
            self.reload_camera_disp_matrix(camera);
        }

        if self.camera.1 != *camera.quater() {
            self.reload_camera_quater(camera);
        }
    }

    pub fn persp_matrix(&self) -> Matrix4<f64> {
        self.persp_matrix.0
    }

    pub fn persp_matrix_size(&self) -> BufferSize {
        self.persp_matrix.1
    }

    pub fn reload_persp_matrix(&mut self, new_size: BufferSize) {
        let matrix = Matrix4::persp_rh_matrix(90.0, new_size.width as f64 / new_size.height as f64, 0.1, 100.0);

        self.persp_matrix = (matrix, new_size);
    }

    pub fn camera_disp_matrix(&self) -> Matrix4<f64> {
        self.camera.0
    }

    pub fn camera_quater(&self) -> Quaternion<f64> {
        self.camera.1
    }

    pub fn camepa_pos(&self) -> Vec3<f64> {
        self.camera.2
    }

    pub fn reload_camera_disp_matrix(&mut self, camera: &Camera) {
        self.camera.0 = camera.to_displacement_matrix();
    }

    pub fn reload_camera_quater(&mut self, camera: &Camera) {
        self.camera.1 = camera.to_rotation_quaternion();
    }

    pub fn camera_pos(&self) -> Vec3<f64> {
        self.camera.2
    }
}
