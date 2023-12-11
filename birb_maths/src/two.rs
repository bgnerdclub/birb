use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Vector<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn fill(value: T) -> Self {
        Self { x: value, y: value }
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul for Vector<T> {
    type Output = Rotor<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Rotor {
            real: (self.x * rhs.x + self.y * rhs.y),
            imaginary: (self.x * rhs.y + self.y * rhs.x),
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> Mul<Rotor<T>> for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: Rotor<T>) -> Self::Output {
        Self {
            x: (self.x * rhs.real - self.y * rhs.imaginary),
            y: (self.x * rhs.imaginary + self.y * rhs.real),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rotor<T> {
    pub real: T,
    pub imaginary: T,
}

impl<T: Neg<Output = T> + Copy> Rotor<T> {
    pub const fn new(real: T, imaginary: T) -> Self {
        Self { real, imaginary }
    }

    pub fn to_row_matrix(&self) -> RowMat<T> {
        RowMat {
            x: Vector {
                x: self.real,
                y: -self.imaginary,
            },
            y: Vector {
                x: self.imaginary,
                y: self.real,
            },
        }
    }

    pub fn to_col_matrix(&self) -> ColMat<T> {
        ColMat {
            x: Vector {
                x: self.real,
                y: self.imaginary,
            },
            y: Vector {
                x: -self.imaginary,
                y: self.real,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowMat<T> {
    pub x: Vector<T>,
    pub y: Vector<T>,
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul<ColMat<T>> for RowMat<T> {
    type Output = Self;

    fn mul(self, rhs: ColMat<T>) -> Self::Output {
        Self {
            x: Vector {
                x: self.x.x * rhs.x.x + self.x.y * rhs.x.y,
                y: self.x.x * rhs.y.x + self.x.y * rhs.y.y,
            },
            y: Vector {
                x: self.y.x * rhs.x.x + self.y.y * rhs.x.y,
                y: self.y.x * rhs.y.x + self.y.y * rhs.y.y,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColMat<T> {
    pub x: Vector<T>,
    pub y: Vector<T>,
}

impl<T> From<ColMat<T>> for RowMat<T> {
    fn from(value: ColMat<T>) -> Self {
        Self {
            x: Vector {
                x: value.x.x,
                y: value.y.x,
            },
            y: Vector {
                x: value.x.y,
                y: value.y.y,
            },
        }
    }
}

impl<T> From<RowMat<T>> for ColMat<T> {
    fn from(value: RowMat<T>) -> Self {
        Self {
            x: Vector {
                x: value.x.x,
                y: value.y.x,
            },
            y: Vector {
                x: value.x.y,
                y: value.y.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let a = Vector::new(1.0, 0.0);
        let b = Vector::new(0.0, 1.0);
        assert_eq!(a + b, Vector::fill(1.0));
        assert_eq!(a - b, Vector::new(1.0, -1.0));
    }

    #[test]
    fn test_rotor() {
        let a = Vector::new(1.0, 0.0);
        let b = Vector::new(0.0, 1.0);
        let i = a * b;
        assert_eq!(
            i,
            Rotor {
                real: 0.0,
                imaginary: 1.0
            }
        );

        assert_eq!(b, a * i);
        let c = Vector::fill(1.0);
        assert_eq!(c * i, Vector::new(-1.0, 1.0));
    }

    #[test]
    fn test_matrix() {
        let i = Rotor {
            real: 0,
            imaginary: 1,
        };
        let row = i.to_row_matrix();
        let col = i.to_col_matrix();
        assert_eq!(
            row * col,
            RowMat {
                x: Vector { x: -1, y: 0 },
                y: Vector { x: 0, y: -1 }
            }
        );
    }
}
