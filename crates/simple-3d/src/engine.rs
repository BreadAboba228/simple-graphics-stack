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
    need_to_redraw: bool,
    mouse_pos: Option<(f32, f32)>,
    is_mouse_pressed: bool
}

impl Engine {
    pub fn new(
        scene: Scene,
        color: Color,
        angles: &[AngleUnit],
        size: BufferSize
    ) -> Self {
        let quater = AngleUnit::unification_to_quater(angles).to_normalized();

        let render_cache = RenderCache::init(scene.shapes(), size, &scene.camera);

        let need_to_redraw = true;

        let mouse_pos = None;

        let is_mouse_pressed = false;

        Self { scene, color, quater, render_cache, need_to_redraw, mouse_pos, is_mouse_pressed }
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
                self.render_cache.reload(buffer.size, &self.scene.camera);

                let shapes = self.scene.shapes();

                // iterate over all vertices and add the rendering cache to the pool
                for (index, shape) in shapes.iter().enumerate() {
                    for vertex in shape.vertexes() {
                        let vertex4 = vertex.into_lifted();

                        // offset camera matrix mul vertex
                        // rotate vertex by camera quater
                        // project vertex into 3d
                        // perspective matrix mul vertex
                        // project vertex into 2d
                        // to real coordinates
                        let vertex3 = self.render_cache.persp_matrix().mul(
                            self.render_cache.camera_disp_matrix()
                                .mul(vertex4)
                                .set_w(0.0)
                                .to_rotated(self.render_cache.camera_quater())
                        )
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
                        buffer.accuracy_draw_line(start, end, self.color);
                    }
                }

                self.render_cache.clear();
            },

            Event::KeyPressed { keys } => {
                self.need_to_redraw = true;

                let quater = self.scene.camera.quater();

                let forward = Vec3::new(0.0, 0.0, 1.0).to_raw_rotated(quater);

                let right = Vec3::new(1.0, 0.0, 0.0).to_raw_rotated(quater);

                //let up = Vec3::new(0.0, 1.0, 0.0).to_raw_rotated(quater);

                let mut move_direction = Vec3::ZERO;

                for key in keys {

                    match key {
                        Key::W => move_direction += forward,

                        Key::S => move_direction -= forward,

                        Key::A => move_direction += right,

                        Key::D => move_direction -= right,

                        Key::Space => {
                            self.scene.camera.pos += Vec3::ZERO.set_y(0.1).to_raw_rotated(self.render_cache.camera_quater());
                        },

                        Key::LeftShift => {
                            self.scene.camera.pos -= Vec3::ZERO.set_y(0.1).to_raw_rotated(self.render_cache.camera_quater());
                        },

                        Key::Up => {
                            self.scene.camera.rotate(&[AngleUnit(Axis::X, -0.5)]);
                        },

                        Key::Down => {
                            self.scene.camera.rotate(&[AngleUnit(Axis::X, 0.5)]);
                        },

                        Key::Left => {
                            self.scene.camera.rotate(&[AngleUnit(Axis::Y, 0.5)]);
                        },

                        Key::Right => {
                            self.scene.camera.rotate(&[AngleUnit(Axis::Y, -0.5)]);
                        },

                        _ => (),
                    }
                }

                move_direction.normalize();

                // 0.1 is move speed
                self.scene.camera.pos += move_direction * 0.1;
            },

            Event::Redrawed => {
                self.need_to_redraw = false
            },

            Event::MousePos { pos } => {

                if self.is_mouse_pressed {

                    let pos_diff = if let Some(curr_pos) = self.mouse_pos {
                        ((pos.0 - curr_pos.0) as f64 * 0.1, (pos.1 - curr_pos.1) as f64 * 0.1)
                    } else {
                    (0.0, 0.0)
                    };

                    self.mouse_pos = Some(pos);

                    let quater_x = Quaternion::from_angle(pos_diff.0.to_radians(), Axis::Y.to_vec());

                    let quater_y = Quaternion::from_angle(-pos_diff.1.to_radians(), Axis::X.to_vec());

                    let quater = quater_x * quater_y;

                    self.scene.camera.raw_rotate(quater);
                }
            },

            Event::MousePressed { button: _, pressed } => {
                self.is_mouse_pressed = pressed;
                if !pressed {
                    self.mouse_pos = None;
                }
            }
        }
    }

    fn need_to_redraw(&self) -> bool {
        self.need_to_redraw
    }
}
