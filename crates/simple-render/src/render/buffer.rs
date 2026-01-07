use std::usize;

use simple_linear_algebra::vector::{vec2::Vec2, vec3::Vec3};

use crate::color::Color;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct BufferSize {
    pub width: usize,
    pub height: usize,
}

impl BufferSize {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub const fn from_get_size(size: (usize, usize)) -> Self {
        Self::new(size.0, size.1)
    }
}

#[derive(Clone)]
pub struct Buffer(pub Vec<u32>);

impl Buffer {
    pub fn new(size: BufferSize) -> Self {
        let vec = vec![0; size.width * size.height];
        Self(vec)
    }

    pub fn fill(&mut self, color: Color) {
        self.0.fill(color.0);
    }

    pub fn draw_point(&mut self, point: Vec2<isize>, color: Color, size: BufferSize) {
        let Vec2 { x, y } = point;

        if (0 <= x && x < size.width as isize) && (0 <= y && y < size.height as isize) {
            self.0[(y as usize) * size.width + x as usize] = color.0;
        }
    }

    pub fn draw_line(
        &mut self,
        size: BufferSize,
        start: Vec2<isize>,
        end: Vec2<isize>,
        color: Color
    ) {
        let Vec2 { x: mut x1, y: mut y1 } = start;
        let Vec2 {x: x2, y: y2} = end;

        let delta_x = (x2 - x1).abs();
        let delta_y = (y2 - y1).abs();

        let step_x =
            if x2 > x1 { 1 }
            else { -1 };

        let step_y =
            if y2 > y1 { 1 }
            else { -1 };

        let mut err = delta_x - delta_y;

        loop {
            self.draw_point(Vec2::new(x1, y1), color, size);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let double_err = err * 2;

            if double_err > -delta_y {
                err -= delta_y;
                x1 += step_x;
            }

            if double_err < delta_x {
                err += delta_x;
                y1 += step_y;
            }
        }
    }

    pub fn draw_triangle(&mut self, triangle: Vec3<Vec2<isize>>, size: BufferSize, color: Color) {
        self.draw_line(size, triangle.x, triangle.y, color);
        self.draw_line(size, triangle.x, triangle.z, color);
        self.draw_line(size, triangle.y, triangle.z, color);
    }

    pub fn is_on_plain(line: Vec2<Vec2<isize>>, point: Vec2<isize>) -> bool {
        let e = (point.x - line.x.x) * (line.y.y - line.x.y) - (point.y - line.x.y) * (line.y.x - line.x.x);
        if e >= 0 {
            true
        } else {
            false
        }
    }

    pub fn is_inside_triangle(triangle: Vec3<Vec2<isize>>, point: Vec2<isize>) -> bool {
        Self::is_on_plain(Vec2::new(triangle.x, triangle.y), point) &&
        Self::is_on_plain(Vec2::new(triangle.y, triangle.z), point) &&
        Self::is_on_plain(Vec2::new(triangle.z, triangle.x), point)
    }

    pub fn fill_triangle(&mut self, triangle: Vec3<Vec2<isize>>, size: BufferSize, color: Color) {
        let x_list = [triangle.x.x, triangle.y.x, triangle.z.x];
        let y_list = [triangle.x.y, triangle.y.y, triangle.z.y];

        let min_x = x_list.iter().min().unwrap();
        let max_x = x_list.iter().max().unwrap();

        let min_y = y_list.iter().min().unwrap();
        let max_y = y_list.iter().max().unwrap();

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                let point = Vec2::new(x, y);

                if Self::is_inside_triangle(triangle, point) {
                    self.draw_point(point, color, size);
                }
            }
        }
    }
}
