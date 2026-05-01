use simple_linear_algebra::vector::{vec2::Vec2, vec3::Vec3};

use crate::{color::Color, render::{image::Image, shader::{ColorFragmentShader, DefaultVertexShader, FragmentShader, ImageFragmentShader, ShaderPipeline, TriangleVertexShader, VertexShader}}};

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

impl From<BufferSize> for Vec2<Vec2<isize>> {
    fn from(value: BufferSize) -> Self {
        Vec2 {
            x: Vec2 { x: 0, y: 0 },
            y: Vec2 { x: value.width as isize - 1, y: value.height as isize - 1 }
        }
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

    pub fn raw_get_point(&self, point: Vec2<isize>) -> u32 {
        let Vec2 { x, y } = point;

        self.raw_buffer.0[(y as usize) * self.size.width + x as usize]
    }

    pub fn get_point(&self, point: Vec2<isize>) -> Option<&u32> {
        let Vec2 { x, y } = point;

        self.raw_buffer.0.get((y as usize) * self.size.width + x as usize)
    }

    pub fn raw_draw_point(&mut self, point: Vec2<isize>, color: Color) {
        let Vec2 { x, y } = point;

        self.raw_buffer.0[(y as usize) * self.size.width + x as usize] = color.0;
    }

    pub fn draw_point(&mut self, point: Vec2<isize>, color: Color) {
        if point.is_inside_buffer(self.size) {
            self.raw_draw_point(point, color);
        }
    }

    pub fn raw_shade_buffer<V: VertexShader, F: FragmentShader>(&mut self, shader: ShaderPipeline<V, F>) {
        self.raw_shade_rect(shader, self.size.into());
    }

    pub fn raw_shade_rect<V: VertexShader, F: FragmentShader>(&mut self, shader: ShaderPipeline<V, F>, rect: Vec2<Vec2<isize>>) {
        for x in rect.x.x..=rect.y.x {
            for y in rect.x.y..=rect.y.y {
                let point = Vec2 { x, y };

                if let Some((point, color)) = shader.shade(point) {
                    if let Some(color) = color {
                        self.raw_draw_point(point, color);
                    }
                }
            }
        }
    }

    pub fn shade_rect<V: VertexShader, F: FragmentShader>(&mut self, shader: ShaderPipeline<V, F>, rect: Vec2<Vec2<isize>>) {
        for x in rect.x.x..=rect.y.x {
            for y in rect.x.y..=rect.y.y {
                let point = Vec2 { x, y };

                if let Some((point, color)) = shader.shade(point) {
                    if let Some(color) = color {
                        self.draw_point(point, color);
                    }
                }
            }
        }
    }

    pub fn accuracy_draw_line(&mut self, start: Vec2<isize>, end: Vec2<isize>, color: Color) {
        let (start, end) = if start.is_inside_buffer(self.size) {
            (start, end)
        } else if end.is_inside_buffer(self.size) {
            (end, start)
        } else {
            return;
        };

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
            let point = Vec2::new(x1, y1);

            if !point.is_inside_buffer(self.size) {
                return;
            }

            self.raw_draw_point(point, color);

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
            self.draw_point(Vec2 { x: x1, y: y1 }, color);

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
        self.accuracy_draw_line(triangle.x, triangle.y, color);
        self.accuracy_draw_line(triangle.x, triangle.z, color);
        self.accuracy_draw_line(triangle.y, triangle.z, color);
    }

    pub fn fill_triangle(&mut self, triangle: Vec3<Vec2<isize>>, color: Color) {
        let x_list = [triangle.x.x, triangle.y.x, triangle.z.x];
        let y_list = [triangle.x.y, triangle.y.y, triangle.z.y];

        let &min_x = x_list.iter().min().unwrap();
        let &max_x = x_list.iter().max().unwrap();

        let &min_y = y_list.iter().min().unwrap();
        let &max_y = y_list.iter().max().unwrap();

        let rect = Vec2::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y));

        let rect = rectangles_intersection(self.size.into(), rect);

        let pipeline = ShaderPipeline {
            vertex: TriangleVertexShader(triangle),
            fragment: ColorFragmentShader(color)
        };

        self.raw_shade_rect(pipeline, rect);
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
        let rect = rectangles_intersection(self.size.into(), rectangle);

        let Vec2 { x: min_x, y: min_y } = rect.x;
        let Vec2 { x: max_x, y: max_y } = rect.y;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.draw_point(Vec2::new(x, y), color);
            }
        }
    }

    pub fn draw_image(&mut self, image: &Image, point: Vec2<isize>) {
        let pipeline = ShaderPipeline {
            vertex: DefaultVertexShader,
            fragment: ImageFragmentShader(image, point),
        };

        let img_point = Vec2 { x: image.0.size.width as isize - 1, y: image.0.size.height as isize - 1 };

        self.shade_rect(pipeline, Vec2 { x: point, y: point + img_point });
    }
}

pub trait Point {
    fn is_inside_rectangle(&self, rectangle: Vec2<Vec2<isize>>) -> bool;

    fn is_inside_triangle(&self, triangle: Vec3<Vec2<isize>>) -> bool;

    fn is_inside_buffer(&self, buffer: BufferSize) -> bool {
        self.is_inside_rectangle(buffer.into())
    }

    fn edge_func(&self, line: Vec2<Vec2<isize>>) -> bool;
}

impl Point for Vec2<isize> {
    fn is_inside_triangle(&self, triangle: Vec3<Vec2<isize>>) -> bool {
        self.edge_func(Vec2::new(triangle.x, triangle.y)) &&
        self.edge_func(Vec2::new(triangle.y, triangle.z)) &&
        self.edge_func(Vec2::new(triangle.z, triangle.x))
    }

    fn edge_func(&self, line: Vec2<Vec2<isize>>) -> bool {
        let e = (self.x - line.x.x) * (line.y.y - line.x.y) - (self.y - line.x.y) * (line.y.x - line.x.x);
        e >= 0
    }

    fn is_inside_rectangle(&self, rectangle: Vec2<Vec2<isize>>) -> bool {
        let Vec2 { x: min_x, y: min_y } = rectangle.x;
        let Vec2 { x: max_x, y: max_y } = rectangle.y;

        (min_x <= self.x && self.x <= max_x) && (min_y <= self.y && self.y <= max_y)
    }
}

pub fn rectangles_intersection(rect_1: Vec2<Vec2<isize>>, rect_2: Vec2<Vec2<isize>>) -> Vec2<Vec2<isize>> {
    let &x1 = [rect_1.x.x, rect_2.x.x].iter().max().unwrap();

    let &y1 = [rect_1.x.y, rect_2.x.y].iter().max().unwrap();

    let &x2 = [rect_1.y.x, rect_2.y.x].iter().min().unwrap();

    let &y2 = [rect_1.y.y, rect_2.y.y].iter().min().unwrap();

    Vec2::new(
        Vec2::new(x1, y1),
        Vec2::new(x2, y2)
    )
}
