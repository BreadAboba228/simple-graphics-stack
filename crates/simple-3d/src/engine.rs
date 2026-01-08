use std::{ops::Mul, sync::{Arc, Mutex}, thread};

use minifb::{Key, Window};
use simple_linear_algebra::{num_traits::Zero, vector::{Axis, Vector, quaternion::Quaternion, vec2::Vec2, vec3::Vec3}};
use simple_render::{color::Color, render::{Render, app_handler::{AppHandler, Event}, buffer::BufferSize, wait}};

use crate::{engine::render_cache::RenderCache, scene::Scene, shape::AngleUnit};

pub mod render_cache;

pub struct Engine {
    scene: Scene,
    color: Color,
    quater: Quaternion<f64>,
    render_cache: RenderCache,
    need_to_redraw: bool
}

impl Engine {
    pub fn new(
        scene: Scene,
        color: Color,
        angles: &[AngleUnit],
        size: BufferSize
    ) -> Self {
        let quater = AngleUnit::unification_to_quater(angles).to_normalized();

        let render_cache = RenderCache::init(scene.shapes(), size);

        let need_to_redraw = true;

        Self { scene, color, quater, render_cache, need_to_redraw }
    }

    pub fn event_loop(&mut self) {
        self.scene.raw_rotate_shapes(self.quater);

        self.need_to_redraw = true;
    }

    pub fn run(self, fps: f64, window: Window) {
        let clone = Arc::new(Mutex::new(self));
        let clone2 = clone.clone();

        let mut render = Render::new(clone, fps, window);

        let tick = 1.0 / fps;

        thread::spawn(move || {
            loop {
                clone2.lock().unwrap()
                    .event_loop();

                wait(tick);
            }
        });

        render.run();
    }

    pub fn to_real(&self, vec: Vec2<f64>, size: BufferSize) -> Vec2<isize> {
        //[-1; 1] + 1 -> [0; 2]
        // [0; 2] / 2 -> [0; 1]
        // [0; 1] * width -> [0; width]
        let x = ((vec.x + 1.0) / 2.0) * size.width as f64;

        //[-1; 1] + 1 -> [0; 2]
        // [0; 2] / 2 -> [0; 1]
        // [1; 0] * height -> [height; 0]
        let y =  (vec.y + 1.0) / 2.0 * size.height as f64;

        Vec2::new(x as isize, y as isize)
    }
}

impl AppHandler for Engine {
    fn event(&mut self, event: Event) {
        match event {
            Event::RedrawReqiest { buffer } => {
                let shapes = self.scene.shapes();

                if self.render_cache.matrix_size() != buffer.size {
                    self.render_cache.reload_matrix(buffer.size);
                }

                for (index, shape) in shapes.iter().enumerate() {
                    for vertex in shape.vertexes() {
                        let vertex4 = vertex.into_lifted();

                        let vertex3 = self.render_cache.matrix().mul(
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
                                .into_vec2(),
                            buffer.size
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
                        buffer.draw_line(start, end, self.color);
                    }
                }

                self.render_cache.clear();
            },

            Event::KeyPressed { key } => {
                self.need_to_redraw = true;
                match key {
                    Key::W => {
                        self.scene.camera.pos += Vec3::ZERO.set_z(0.1).to_raw_rotated(*self.scene.camera.quater());
                    }

                    Key::S => {
                        self.scene.camera.pos -= Vec3::ZERO.set_z(0.1).to_raw_rotated(*self.scene.camera.quater());
                    }

                    Key::A => {
                        self.scene.camera.pos += Vec3::ZERO.set_x(0.1).to_raw_rotated(*self.scene.camera.quater())
                    }

                    Key::D => {
                        self.scene.camera.pos -= Vec3::ZERO.set_x(0.1).to_raw_rotated(*self.scene.camera.quater());
                    }

                    Key::Space => {
                        self.scene.camera.pos += Vec3::ZERO.set_y(0.1).to_raw_rotated(*self.scene.camera.quater());
                    }

                    Key::LeftShift => {
                        self.scene.camera.pos -= Vec3::ZERO.set_y(0.1).to_raw_rotated(*self.scene.camera.quater());
                    }

                    Key::Up => {
                        self.scene.camera.rotate(&[AngleUnit(Axis::X, -0.5)]);
                    }

                    Key::Down => {
                        self.scene.camera.rotate(&[AngleUnit(Axis::X, 0.5)]);
                    }

                    Key::Left => {
                        self.scene.camera.rotate(&[AngleUnit(Axis::Y, 0.5)]);
                    }

                    Key::Right => {
                        self.scene.camera.rotate(&[AngleUnit(Axis::Y, -0.5)]);
                    }

                    _ => (),
                }
            }
        }
    }

    fn need_to_redraw(&self) -> bool {
        self.need_to_redraw
    }

    fn redrawed(&mut self) {
        self.need_to_redraw = false;
    }
}
