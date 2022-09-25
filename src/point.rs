use gdnative::prelude::*;
use super::utils::*;

use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct FixedVec2 {
    pub x: FixedNum,
    pub y: FixedNum,
}

impl FixedVec2 {
    pub fn new() -> Self {
        FixedVec2::coords(0, 0)
    }

    pub fn coords<T: SimNum>(x: T, y: T) -> Self {
        FixedVec2 {
            x: x.to_fixed(),
            y: y.to_fixed(),
        }
    }
}

impl Default for FixedVec2 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<FixedVec2> for Vector2 {
    fn from(p: FixedVec2) -> Vector2 {
        Vector2::new(p.x.to_num(), p.y.to_num())
    }
}

impl Add<FixedVec2> for FixedVec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<FixedVec2> for FixedVec2 {
    type Output = FixedVec2;
    fn sub(self, rhs: FixedVec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<FixedVec2> for FixedVec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<FixedVec2> for FixedVec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> Mul<T> for FixedVec2 where T: SimNum + Copy {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.to_fixed::<FixedNum>(),
            y: self.y * rhs.to_fixed::<FixedNum>(),
        }
    }
}

impl<T> Div<T> for FixedVec2 where T: SimNum + Copy {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.to_fixed::<FixedNum>(),
            y: self.y / rhs.to_fixed::<FixedNum>(),
        }
    }
}

impl PartialEq<FixedVec2> for FixedVec2 {
    fn eq(&self, rhs: &FixedVec2) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

