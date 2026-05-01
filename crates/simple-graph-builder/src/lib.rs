use std::{sync::{Arc, Mutex}, thread};

use minifb::Window;
use simple_linear_algebra::vector::vec2::Vec2;
use simple_render::{color::Color, render::{Render, app_handler::{AppHandler, Event}}};

use crate::parser::{Lexer, Parser};

pub struct Builder {
    func: Box<dyn Fn(isize) -> isize>,
    color: Color,
    need_to_redraw: bool
}

pub mod parser;

impl Builder {
    pub fn new(func: Box<dyn Fn(isize) -> isize>, color: Color) -> Self {
        Self { func, color, need_to_redraw: true }
    }

    pub fn run(self, fps: f64, window: Window) {

        let clone = Arc::new(Mutex::new(self));

        let clone2 = clone.clone();

        let mut render = Render::new(clone, fps, window);

        render.run();

        let _handle = thread::spawn(||
            loop {
                let mut str = String::new();

                std::io::stdin()
                    .read_line(&mut str)
                    .unwrap();

                let mut lexer = Lexer::new(&str);

                let tokens = match lexer.tokenize() {
                    Ok(tokens) => tokens,

                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                };

                let mut parser = Parser::new(&tokens);

                let expr = match parser.parse() {
                    Ok(expr) => expr,

                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                };

                (clone2.lock().unwrap().func) = move |x: isize| expr.eval(x as f64) as isize;
            }
        );
    }
}

impl<F: Fn(isize) -> isize> AppHandler for Builder<F> {
    fn event(&mut self, event: Event) {
        match event {
            Event::RedrawReqiest { buffer } => {
                buffer.fill(Color::BLACK);

                let mut vec2_vec = Vec::<Vec2<isize>>::with_capacity(buffer.size.width);

                for x in 0..buffer.size.width as isize {
                    let y = (buffer.size.height as isize - 1) - (self.func)(x);

                    let vec2 = Vec2::new(x, y / 2);

                    vec2_vec.push(vec2);
                }

                let mut iter = vec2_vec.iter().peekable();

                while let Some(vec) = iter.next() {
                    if let Some(&f) = iter.peek() {
                        buffer.accuracy_draw_line(*vec, *f, self.color);
                    }
                }
            }

            Event::Redrawed => {
                self.need_to_redraw = false;
            }

            _ => ()
        }
    }

    fn need_to_redraw(&self) -> bool {
        self.need_to_redraw
    }
}
