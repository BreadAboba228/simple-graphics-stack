use std::{thread, time::Duration};

use crate::core::{figura::{Figura, AngleUnit}, vector::Vec3};

pub mod figura;
pub mod vector;

#[derive(Clone, Copy)]
pub struct BufferSize {
    pub height: usize,
    pub width: usize
}

impl BufferSize {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }
}

pub struct Buffer<'a>(&'a[&'a[char]]);

impl<'a> Buffer<'a> {
    pub fn new(const height: usize, width: usize) -> Vec<Vec<char>> {
        let mut height_buffer = [0; height];
        let mut vector = Vec::<Vec<char>>::with_capacity(buffer_size.height);
        for _i in 0..buffer_size.height {
            let mut vec2 = Vec::<char>::with_capacity(buffer_size.width);
            for _j in 0..buffer_size.width {
                vec2.push(' ');
            }
            vector.push(vec2);
        }
        vector
    }
}

fn clear_console() {
    {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("OS error");
    }
}

fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    pub fn from_vec3(
        vector: &Vec3,
        buffer_size: &BufferSize,
        scale: i32
    ) -> Self {
        Self {
            x: (vector.x * scale as f64 + (buffer_size.width / 2) as f64) as i32,
            y: (vector.y * scale as f64 / 2.0 + (buffer_size.height / 2) as f64 ) as i32
        }
    }
}

pub fn draw_line(
    buffer: &mut [Vec<char>],
    start: Vec2,
    end: Vec2,
    buffer_size: &BufferSize,
    symbol: char
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
        if (0 <= x1 && x1 < buffer_size.width as i32) &&
        (0 <= y1 && y1 < buffer_size.height as i32) {
            buffer[y1 as usize][x1 as usize] = symbol
        }

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

pub fn draw(vec: &[&[char]]) {
    for i in vec {
        for &j in i {
            print!("{}", j);
        }
        println!("");
    }
}

pub struct Engine<'a> {
    figura: Figura, 
    symbol: char,
    tick: f64,
    buffer_size: BufferSize,
    angles: &'a[AngleUnit],
}

impl<'a> Engine<'a> {
    pub const fn new(
        figura: Figura, 
        symbol: char, 
        tick: f64, 
        buffer_size: BufferSize,
        angles: &'a[AngleUnit]
    ) -> Self {
        Self { figura, symbol, tick, buffer_size, angles }
    }

    pub fn render_frame(&self) -> &[&[char]] {
        let mut buffer = Buffer::new(self.buffer_size);

        for edge in self.figura.edges() {
            let start = Vec2::from_vec3(
                &self.figura.vertexes()[edge.0], 
                &self.buffer_size,
                10
            );

            let end = Vec2::from_vec3(
                &self.figura.vertexes()[edge.1],
                &self.buffer_size,
                10
            );

            draw_line(&mut buffer, start, end, &self.buffer_size, self.symbol);
    }

    buffer
}

    pub fn run(&mut self) {
        let secs = 1.0 / self.tick;
        loop {
            draw(&self.render_frame());

            self.figura.rotate(&self.angles);

            wait(secs);

            clear_console();
        }
    }
}