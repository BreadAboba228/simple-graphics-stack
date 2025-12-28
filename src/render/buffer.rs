use simple_linear_algebra_rs::vector::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct BufferSize {
    pub height: usize,
    pub width: usize,
}

impl BufferSize {
    pub const fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }
}

pub struct Buffer(pub Vec<Vec<char>>);

impl Buffer {
    pub fn new(size: BufferSize) -> Self {
        let mut height_vec = Vec::<Vec<char>>::with_capacity(size.height);
        for _ in 0..size.height {
            let width_vec = vec![' '; size.width];
            height_vec.push(width_vec);
        }
        Self(height_vec)
    }

    pub fn clear(&mut self) {
        for i in &mut self.0 {
            i.fill(' ');
        }
    }

    pub fn display(&self) {
        for i in &self.0 {
            for &j in i {
                print!("{j}");
            }
            print!("\n");
        }
    }

    pub fn draw_line(
        &mut self,
        size: BufferSize,
        start: Vec2<isize>,
        end: Vec2<isize>,
        ch: char
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
            if (0 <= x1 && x1 < size.width as isize) &&
            (0 <= y1 && y1 < size.height as isize) {
                self.0[y1 as usize][x1 as usize] = ch
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
}
