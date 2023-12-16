#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub mod two;

pub trait Zero {
    fn zero() -> Self;
}

pub trait Unit {
    fn unit() -> Self;
}

impl Zero for u8 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u16 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for usize {
    fn zero() -> Self {
        0
    }
}

impl Zero for i8 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for isize {
    fn zero() -> Self {
        0
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl Unit for u8 {
    fn unit() -> Self {
        1
    }
}

impl Unit for u16 {
    fn unit() -> Self {
        1
    }
}
impl Unit for u32 {
    fn unit() -> Self {
        1
    }
}

impl Unit for u64 {
    fn unit() -> Self {
        1
    }
}

impl Unit for usize {
    fn unit() -> Self {
        1
    }
}

impl Unit for i8 {
    fn unit() -> Self {
        1
    }
}

impl Unit for i16 {
    fn unit() -> Self {
        1
    }
}
impl Unit for i32 {
    fn unit() -> Self {
        1
    }
}

impl Unit for i64 {
    fn unit() -> Self {
        1
    }
}

impl Unit for isize {
    fn unit() -> Self {
        1
    }
}

impl Unit for f32 {
    fn unit() -> Self {
        1.0
    }
}

impl Unit for f64 {
    fn unit() -> Self {
        1.0
    }
}
