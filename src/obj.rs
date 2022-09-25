use gdnative::prelude::*;

use super::point::*;

pub trait ObjectData: ToVariant {}

pub trait GameObject<T: ObjectData> {
    fn tick(&mut self);
    fn get_data(&self) -> T;
    fn get_position(&self) -> FixedVec2;
    fn set_position(&mut self, point: FixedVec2);
}

pub struct PhysicsObject {
    point: FixedVec2,
    vel: FixedVec2,
    accel: FixedVec2,
}