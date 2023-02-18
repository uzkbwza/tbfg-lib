use gdnative::prelude::*;
use fixed_trigonometry::*;
use atan::atan2;
use super::utils::*;
use fixed_sqrt::*;

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::PartialEq;

#[derive(ToVariant, FromVariant, Clone, Debug)]
pub struct FixedVec2String {
    pub x: String,
    pub y: String,
}

#[derive(ToVariant, FromVariant, Clone, Debug)]
pub struct FixedVec2Int {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct FixedVec2 {
    pub x: FixedNum,
    pub y: FixedNum,
}

impl FixedVec2 {
    pub fn new() -> Self {
        FixedVec2::coords(0, 0)
    }

    pub fn coords<A: SimNum, B: SimNum>(x: A, y: B) -> Self {
        FixedVec2 {
            x: x.to_fixed(),
            y: y.to_fixed(),
        }
    }

    pub fn from_string(x: String, y: String) -> Self {
        FixedVec2 {
            x: FixedNum::unwrapped_from_str(&x),
            y: FixedNum::unwrapped_from_str(&y),
        }
    }

    pub fn to_string(&self) -> FixedVec2String    {
        FixedVec2String {
            x: self.x.to_string(),
            y: self.y.to_string(),
        }
    }

    pub fn to_int(&self) -> FixedVec2Int {
        FixedVec2Int {
            x: self.x.round().to_num(),
            y: self.y.round().to_num()
        }
    }

    pub fn length(&self) -> FixedNum {
        (self.x.abs().powu(2) + self.y.abs().powu(2)).sqrt()
    }

    pub fn length_squared(&self) -> FixedNum {
        self.x.abs().powu(2) + self.y.abs().powu(2)
    }

    pub fn angle(&self) -> FixedNum {
        atan2(self.y, self.x)
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();

        Self {
            x: if length > 0 { self.x / length } else { FixedNum::from_num(0) },
            y: if length > 0 { self.y / length } else { FixedNum::from_num(0) },
        }
    }

    pub fn round(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
    }

    pub fn rounded(&self) -> Self {
        let mut new = *self;
        new.round();
        new
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

impl<T> Mul<T> for FixedVec2 where T: SimNum {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.to_fixed::<FixedNum>(),
            y: self.y * rhs.to_fixed::<FixedNum>(),
        }
    }
}

impl<T> Div<T> for FixedVec2 where T: SimNum {
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

impl AddAssign<FixedVec2> for FixedVec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign<FixedVec2> for FixedVec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl MulAssign<FixedVec2> for FixedVec2 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl<T> MulAssign<T> for FixedVec2 where T: SimNum {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other.to_fixed::<FixedNum>(),
            y: self.y * other.to_fixed::<FixedNum>(),
        };
    }
}

impl DivAssign<FixedVec2> for FixedVec2 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl<T> DivAssign<T> for FixedVec2 where T: SimNum {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other.to_fixed::<FixedNum>(),
            y: self.y / other.to_fixed::<FixedNum>(),
        };
    }
}