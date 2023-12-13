use std::any::Any;
use std::ops::{Add, Mul, Sub};

pub trait UiDrawable {
    fn get_size(&self);
}

#[cfg(test)]
mod tests {}
