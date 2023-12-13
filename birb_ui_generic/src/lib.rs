use std::any::Any;
use std::ops::{Add, Mul, Sub};

pub trait UiDrawable {
    fn get_size(&self) -> UDim2;
}

pub struct Vector2<T> {
    x: T,
    y: T,
}

impl Add for Vector2<u8> {
    type Output = Vector2<u8>;
    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl Sub for Vector2<u8> {
    type Output = Vector2<u8>;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}
impl Mul for Vector2<u8> {
    type Output = Vector2<u8>;
    fn mul(self, rhs: u8) -> Self::Output {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

pub struct UDim2 {
    offset: Vector2<u16>,
    scale: Vector2<f32>,
}

#[cfg(test)]
mod tests {}
