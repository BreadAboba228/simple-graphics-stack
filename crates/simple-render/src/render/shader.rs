use simple_linear_algebra::vector::{vec2::Vec2, vec3::Vec3};

use crate::{color::Color, render::{buffer::Point, image::Image}};

pub struct ShaderPipeline<V: VertexShader, F: FragmentShader> {
    pub vertex: V,
    pub fragment: F,
}

impl<V: VertexShader, F: FragmentShader> ShaderPipeline<V, F> {
    pub fn shade(&self, input: Vec2<isize>) -> Option<(Vec2<isize>, Option<Color>)> {
        if let Some(point) = self.vertex.shade(input) {
            let color = self.fragment.shade(point);
            Some((point, color))
        } else {
            None
        }
    }
}

pub trait VertexShader {
    fn shade(&self, input: Vec2<isize>) -> Option<Vec2<isize>>;
}

pub trait FragmentShader {
    fn shade(&self, input: Vec2<isize>) -> Option<Color>;
}

pub struct DefaultVertexShader;

impl VertexShader for DefaultVertexShader {
    fn shade(&self, input: Vec2<isize>) -> Option<Vec2<isize>> {
        Some(input)
    }
}

pub struct TriangleVertexShader(pub Vec3<Vec2<isize>>);

impl VertexShader for TriangleVertexShader {
    fn shade(&self, input: Vec2<isize>) -> Option<Vec2<isize>> {
        if input.is_inside_triangle(self.0) {
            Some(input)
        } else {
            None
        }
    }
}

pub struct ColorFragmentShader(pub Color);

impl FragmentShader for ColorFragmentShader {
    fn shade(&self, _input: Vec2<isize>) -> Option<Color> {
        Some(self.0)
    }
}

pub struct ImageFragmentShader<'a>(pub &'a Image, pub Vec2<isize>);

impl<'a> FragmentShader for ImageFragmentShader<'a> {
    fn shade(&self, input: Vec2<isize>) -> Option<Color> {
        let img_point = input - self.1;

        if let Some(color) = self.0.0.get_point(img_point) {
            Some(Color(*color))
        } else {
            None
        }
    }
}
