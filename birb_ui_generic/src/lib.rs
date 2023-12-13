use std::any::Any;
use std::ops::{Add, Mul, Sub};

pub trait UiDrawable {
    fn get_size(&self) -> UDim2;
}

#[cfg(test)]
mod tests {}
