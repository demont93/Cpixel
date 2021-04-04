use crate::Dimensions;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Point<T> {
    pub y: T,
    pub x: T,
}

impl<T: PartialOrd> Point<T> {
    pub fn any_greater(&self, other: &Self) -> bool {
        self.x > other.x || self.y > other.y
    }

    pub fn any_lesser(&self, other: &Self) -> bool {
        self.x < other.x || self.y < other.y
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: From<usize>> From<Dimensions> for Point<T> {
    fn from(dimensions: Dimensions) -> Self {
        Self {
            y: dimensions.height.into(),
            x: dimensions.width.into(),
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}