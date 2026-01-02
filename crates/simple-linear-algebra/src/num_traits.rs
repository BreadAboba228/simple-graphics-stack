pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_zero_trait {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0 as $t;
            }
        )*
    };
}

impl_zero_trait!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

pub trait One {
    const ONE: Self;
}

macro_rules! impl_one_trait {
    ($($t:ty),*) => {
        $(
            impl One for $t {
                const ONE: Self = 1 as $t;
            }
        )*
    };
}

impl_one_trait!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

pub trait Two {
    const TWO: Self;
}

macro_rules! impl_two_trait {
    ($($t:ty),*) => {
        $(
            impl Two for $t {
                const TWO: Self = 2 as $t;
            }
        )*
    };
}

impl_two_trait!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

pub trait NegOne {
    const NEG_ONE: Self;
}

macro_rules! impl_neg_one_trait {
    ($($t:ty),*) => {
        $(
            impl NegOne for $t {
                const NEG_ONE: Self = -1 as $t;
            }
        )*
    };
}

impl_neg_one_trait!(i8, i16, i32, i64, i128, isize, f32, f64);

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
