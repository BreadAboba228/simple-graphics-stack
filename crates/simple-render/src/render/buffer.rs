use std::usize;

use simple_linear_algebra::vector::vec2::Vec2;

use crate::color::Color;

#[derive(Clone, Copy)]
pub struct BufferSize {
    pub width: usize,
    pub height: usize,
}

impl BufferSize {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

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
}
