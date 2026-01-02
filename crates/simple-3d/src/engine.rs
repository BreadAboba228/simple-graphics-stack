use std::ops::Mul;

use minifb::Window;
use simple_linear_algebra::{matrix::matrix4::Matrix4, vector::{Vector, quaternion::Quaternion, vec2::Vec2}};
use simple_render::{color::Color, render::{Render, app_handler::AppHandler, buffer::{Buffer, BufferSize}}};

use crate::{engine::render_cache::RenderCache, scene::Scene, shape::AngleUnit};

pub mod render_cache;

pub struct Engine {
    scene: Scene,
    color: Color,
    buffer_size: BufferSize,
    quater: Quaternion<f64>,
    matrix: Matrix4<f64>,
    render_cache: RenderCache
}

impl Engine {
    pub fn new(
        scene: Scene,
        color: Color,
        buffer_size: BufferSize,
        angles: &[AngleUnit],
        matrix: Matrix4<f64>
    ) -> Self {
        let quater = AngleUnit::unification_to_quater(angles).to_normalized();

        let render_cache = RenderCache::init(scene.shapes());

        Self { scene, color, buffer_size, quater, matrix, render_cache }
    }

    pub fn run(&mut self, fps: f64, window: Window) {
        let mut render = Render::new(self, fps, window);

        render.run();
    }

    pub fn to_real(&self, vec: Vec2<f64>) -> Vec2<isize> {
        //[-1; 1] + 1 -> [0; 2]
        // [0; 2] / 2 -> [0; 1]
        // [0; 1] * width -> [0; width]
        let x = ((vec.x + 1.0) / 2.0) * self.buffer_size.width as f64;

        //[-1; 1] + 1 -> [0; 2]
        // [0; 2] / 2 -> [0; 1]
        // 1.0 - [0; 1] -> [1; 0] (axis reversal)
        // [1; 0] * height -> [height; 0]
        let y =  (1.0 - (vec.y + 1.0) / 2.0) * self.buffer_size.height as f64;

        Vec2::new(x as isize, y as isize)
    }
}

impl AppHandler for Engine {
    fn redraw(&mut self, buffer: &mut Buffer) {
        let shapes = self.scene.shapes();

        for (index, shape) in shapes.iter().enumerate() {
            for vertex in shape.vertexes() {
                let vertex4 = vertex.into_lifted();

                let vertex3 = self.matrix.mul(
                    self.scene.camera
                        .to_displacement_matrix()
                        .mul(vertex4)
                        .set_w(0.0)
                        .to_rotated(self.scene.camera.to_rotation_quaternion())
                        .set_w(1.0)
                )
                    .to_projected()
                    .into_vec3();

                let vertex2 = self.to_real(
                    vertex3
                        .to_projected()
                        .into_vec2()
                );

                self.render_cache.push(index, vertex2);
            }
        }

        buffer.fill(Color::new(0));

        for (index, shape) in self.scene.shapes().iter().enumerate() {
            for edge in shape.edges() {
                let start = self.render_cache.get(index, edge.0);

                let end = self.render_cache.get(index, edge.1);

                //TODO: replace isize with usize in draw_line
                buffer.draw_line(self.buffer_size, start, end, self.color);
            }
        }

        self.scene.raw_rotate_shapes(self.quater);

        self.render_cache.clear();
    }


    fn buffer_size(&self) -> BufferSize {
        self.buffer_size
    }
}
