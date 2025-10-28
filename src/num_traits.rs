pub trait Consts {
    const ZERO: Self;
    const ONE: Self;
}

impl Consts for i8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for i16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for i64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for i128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for u128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for isize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl Consts for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Consts for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

pub trait Two {
    const TWO: Self;
}

impl Two for i8 {
    const TWO: Self = 2;
}

impl Two for u8 {
    const TWO: Self = 2;
}

impl Two for i16 {
    const TWO: Self = 2;
}

impl Two for u16 {
    const TWO: Self = 2;
}

impl Two for i32 {
    const TWO: Self = 2;
}

impl Two for u32 {
    const TWO: Self = 2;
}

impl Two for i64 {
    const TWO: Self = 2;
}

impl Two for u64 {
    const TWO: Self = 2;
}

impl Two for i128 {
    const TWO: Self = 2;
}

impl Two for u128 {
    const TWO: Self = 2;
}

impl Two for isize {
    const TWO: Self = 2;
}

impl Two for usize {
    const TWO: Self = 2;
}

impl Two for f32 {
    const TWO: Self = 2.0;
}

impl Two for f64 {
    const TWO: Self = 2.0;
}

pub trait NegOne {
    const NEG_ONE: Self;
}

impl NegOne for i8 {
    const NEG_ONE: Self = -1;
}

impl NegOne for i16 {
    const NEG_ONE: Self = -1;
}

impl NegOne for i32 {
    const NEG_ONE: Self = -1;
}

impl NegOne for i64 {
    const NEG_ONE: Self = -1;
}

impl NegOne for i128 {
    const NEG_ONE: Self = -1;
}

impl NegOne for isize {
    const NEG_ONE: Self = -1;
}

impl NegOne for f32 {
    const NEG_ONE: Self = -1.0;
}

impl NegOne for f64 {
    const NEG_ONE: Self = -1.0;
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait SinCos {
    fn sin(self) -> Self;
    
    fn cos(self) -> Self;

    fn sin_cos(self) -> (Self, Self) where Self: Sized;
}

impl SinCos for f32 {
    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
}

impl SinCos for f64 {
    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
}