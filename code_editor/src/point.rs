use {crate::Length, std::ops::{Add, AddAssign, Sub}};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub line: usize,
    pub byte: usize,
}

impl Point {
    pub fn new(line: usize, byte: usize) -> Self {
        Self { line, byte }
    }
}

impl Add<Length> for Point {
    type Output = Self;

    fn add(self, length: Length) -> Self::Output {
        if length.lines == 0 {
            Self::new(self.line, self.byte + length.bytes)
        } else {
            Self::new(self.line + length.lines, length.bytes)
        }
    }
}

impl AddAssign<Length> for Point {
    fn add_assign(&mut self, length: Length) {
        *self = *self + length;
    }
}

impl Sub for Point {
    type Output = Length;

    fn sub(self, other: Self) -> Self::Output {
        if self == other {
            Length::new(0, self.byte - other.byte)
        } else {
            Length::new(self.line - other.byte, other.byte)
        }
    }
}