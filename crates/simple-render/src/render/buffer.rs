use simple_linear_algebra::vector::{vec2::Vec2, vec3::Vec3};

use crate::{color::Color, render::image::Image};

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
pub struct RawBuffer(pub Vec<u32>);

impl RawBuffer {
    pub fn new(vec: Vec<u32>) -> Self {
        RawBuffer(vec)
    }

    pub fn init(size: BufferSize) -> Self {
        let vec = vec![0; size.width * size.height];
        Self::new(vec)
    }

    pub fn fill(&mut self, color: Color) {
        self.0.fill(color.0);
    }

    pub fn edge_func(line: Vec2<Vec2<isize>>, point: Vec2<isize>) -> bool {
        let e = (point.x - line.x.x) * (line.y.y - line.x.y) - (point.y - line.x.y) * (line.y.x - line.x.x);
        if e >= 0 {
            true
        } else {
            false
        }
    }

    pub fn is_inside_triangle(triangle: Vec3<Vec2<isize>>, point: Vec2<isize>) -> bool {
        Self::edge_func(Vec2::new(triangle.x, triangle.y), point) &&
        Self::edge_func(Vec2::new(triangle.y, triangle.z), point) &&
        Self::edge_func(Vec2::new(triangle.z, triangle.x), point)
    }
}

#[derive(Clone)]
pub struct Buffer {
    pub raw_buffer: RawBuffer,
    pub size: BufferSize
}

impl Buffer {
    pub fn new(raw_buffer: RawBuffer, size: BufferSize) -> Self {
        Self { raw_buffer, size }
    }

    pub fn init(size: BufferSize) -> Self {
        let raw_buffer = RawBuffer::init(size);

        Self::new(raw_buffer, size)
    }

    pub fn fill(&mut self, color: Color) {
        self.raw_buffer.fill(color);
    }

    pub fn get_point(&self, point: Vec2<isize>) -> u32 {
        let Vec2 { x, y } = point;

        self.raw_buffer.0[(y as usize) * self.size.width + x as usize]
    }

    pub fn draw_point(&mut self, point: Vec2<isize>, color: Color) {
        let Vec2 { x, y } = point;

        if (0 <= x && x < self.size.width as isize) && (0 <= y && y < self.size.height as isize) {
            self.raw_buffer.0[(y as usize) * self.size.width + x as usize] = color.0;
        }
    }

    pub fn draw_line(&mut self, start: Vec2<isize>, end: Vec2<isize>, color: Color) {
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
            self.draw_point(Vec2::new(x1, y1), color);

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

    pub fn draw_triangle(&mut self, triangle: Vec3<Vec2<isize>>, color: Color) {
        self.draw_line(triangle.x, triangle.y, color);
        self.draw_line(triangle.x, triangle.z, color);
        self.draw_line(triangle.y, triangle.z, color);
    }

    pub fn fill_triangle(&mut self, triangle: Vec3<Vec2<isize>>, color: Color) {
        let x_list = [triangle.x.x, triangle.y.x, triangle.z.x];
        let y_list = [triangle.x.y, triangle.y.y, triangle.z.y];

        let min_x = x_list.iter().min().unwrap();
        let max_x = x_list.iter().max().unwrap();

        let min_y = y_list.iter().min().unwrap();
        let max_y = y_list.iter().max().unwrap();

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                let point = Vec2::new(x, y);

                if RawBuffer::is_inside_triangle(triangle, point) {
                    self.draw_point(point, color);
                }
            }
        }
    }

    pub fn draw_rectangle(&mut self, rectangle: Vec2<Vec2<isize>>, color: Color) {
        let Vec2 { x: min_x, y: min_y } = rectangle.x;
        let Vec2 { x: max_x, y: max_y } = rectangle.y;

        for x in min_x..=max_x {
            self.draw_point(Vec2::new(x, min_y), color);
            self.draw_point(Vec2::new(x, max_y), color);
        }

        for y in min_y..=max_y {
            self.draw_point(Vec2::new(y, min_x), color);
            self.draw_point(Vec2::new(y, max_x), color);
        }
    }

    pub fn fill_rectangle(&mut self, rectangle: Vec2<Vec2<isize>>, color: Color) {
        let Vec2 { x: min_x, y: min_y } = rectangle.x;
        let Vec2 { x: max_x, y: max_y } = rectangle.y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.draw_point(Vec2::new(x, y), color);
            }
        }
    }

    pub fn draw_image(&mut self, image: &Image, point: Vec2<isize>) {
        let Vec2 { x: offset_x, y: offset_y } = point;
        let (img_x, img_y) = (image.0.size.width as isize, image.0.size.height as isize);

        for y in 0..img_y {
            for x in 0..img_x {
                let buf_point = Vec2::new(x + offset_x, y + offset_y);

                let img_point = Vec2::new(x, y);

                self.draw_point(buf_point, Color::new(image.0.get_point(img_point)));
            }
        }
    }
}
