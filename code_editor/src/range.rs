use crate::{Length, Point};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Range {
    pub start: Point,
    pub end: Point,
}

impl Range {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length() == Length::default()
    }

    pub fn length(&self) -> Length {
        self.end - self.start
    }
}