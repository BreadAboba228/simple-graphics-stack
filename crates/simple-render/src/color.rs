use std::{ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign}, u32};

#[derive(Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const fn new(color: u32) -> Self {
        Self(color)
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        let color = (r << 16) | (g << 8) | b;
        Self::new(color)
    }

    pub const fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.0 >> 16) as u8, (self.0 >> 8) as u8, self.0 as u8)
    }

    pub const BLACK: Self = Color::new(0);
    pub const WHITE: Self = Color::new(u32::MAX);
    pub const RED: Self = Color::from_rgb(255, 0, 0);
    pub const GREEN: Self = Color::from_rgb(0, 255, 0);
    pub const BLUE: Self = Color::from_rgb(0, 0, 255);
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.to_rgb();
        let (r2, g2, b2) = rhs.to_rgb();

        let r = r1.saturating_add(r2);
        let g = g1.saturating_add(g2);
        let b = b1.saturating_add(b2);

        Self::from_rgb(r, g, b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.to_rgb();
        let (r2, g2, b2) = rhs.to_rgb();

        let r = r1.saturating_sub(r2);
        let g = g1.saturating_sub(g2);
        let b = b1.saturating_sub(b2);

        Self::from_rgb(r, g, b)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.to_rgb();
        let (r2, g2, b2) = rhs.to_rgb();

        let (r1, g1, b1) = (r1 as u16, g1 as u16, b1 as u16);
        let (r2, g2, b2) = (r2 as u16, g2 as u16, b2 as u16);

        let r = (r1 * r2) / 255;
        let g = (g1 * g2) / 255;
        let b = (b1 * b2) / 255;

        Self::from_rgb(r as u8, g as u8, b as u8)
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.to_rgb();
        let (r2, g2, b2) = rhs.to_rgb();

        let div = |a: u8, b: u8| -> u8 {
            if b == 0 {
                255
            } else {
                ((a as u16 * 255) / b as u16).min(255) as u8
            }
        };

        let r = div(r1, r2);
        let g = div(g1, g2);
        let b = div(b1, b2);

        Self::from_rgb(r, g, b)
    }
}
