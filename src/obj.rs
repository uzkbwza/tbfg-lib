use gdnative::prelude::*;

use super::point::*;
use super::utils::*;

const PUSHBACK_FRIC: FixedNum = FixedNum::unwrapped_from_str("0.55");
const MAX_PUSHBACK: FixedNum = FixedNum::unwrapped_from_str("10");

#[derive(ToVariant, FromVariant, Clone)]
pub struct BaseObjectData {
    position_float: Vector2,
    // vel_float: Vector2,
    // accel_float: Vector2,
    position_x: i32,
    position_y: i32,
    vel_x: String,
    vel_y: String,
    facing: isize,
    grounded: bool,
}

#[derive(ToVariant, FromVariant, Clone, Copy, Debug)]
pub enum Facing {
    Left = -1,
    Right = 1,
}

// #[derive(NativeClass)]
// #[inherit(Reference)]
#[derive(Clone, Debug)]
pub struct BaseObject {
    pub facing: Facing,
    pub grounded: bool,
    pub pos: FixedVec2,
    pub vel: FixedVec2,
    pub accel: FixedVec2,
    pub pushback_vel: FixedNum,
    pub gravity: FixedNum,
    pub ground_friction: FixedNum,
    pub air_friction: FixedNum,
    pub max_ground_speed: FixedNum,
    pub max_air_speed: FixedNum,
    pub max_fall_speed: FixedNum,
}

impl Default for BaseObject {
    fn default() -> Self {
        Self::new()
    }
}

// #[methods]
impl BaseObject {
    pub fn new() -> Self {
        BaseObject {
            facing: Facing::Right,
            grounded: true,
            pos: FixedVec2::new(),
            vel: FixedVec2::new(),
            accel: FixedVec2::new(),
            pushback_vel: FixedNum::from_num(0),
            gravity: FixedNum::from_num(1),
            ground_friction: FixedNum::from_num(1),
            air_friction: FixedNum::from_num(1),
            max_ground_speed: FixedNum::from_num(1),
            max_air_speed: FixedNum::from_num(1),
            max_fall_speed: FixedNum::from_num(1),
        }
    }

    pub fn get_data(&self) -> BaseObjectData {
        BaseObjectData {
            position_float: self.pos.rounded().into(),
            position_x: self.pos.x.round().to_num(),
            position_y: self.pos.y.round().to_num(),
            vel_x: self.vel.x.to_string(),
            vel_y: self.vel.y.to_string(),
            facing: self.facing as isize,
            grounded: self.grounded,
        }
    }

    pub fn set_facing_int(&mut self, facing: i32) {
        self.facing = match facing {
            -1 => Facing::Left,
            1 => Facing::Right,
            _ => panic!("invalid facing direction"),
        };
    }

    pub fn set_vel(&mut self, vel: FixedVec2) {
        self.vel = vel;
    }

    pub fn set_facing(&mut self, facing: Facing) {
        self.facing = facing;
    }

    pub fn move_directly(&mut self, v: FixedVec2) {
        self.set_position(self.pos + v);
    }

    pub fn move_directly_relative(&mut self, v: FixedVec2) {
        self.move_directly(self.get_relative_vec(v))
    }

    pub fn add_pushback(&mut self, pushback: FixedNum) {
        if self.pushback_vel.abs() < MAX_PUSHBACK {
            self.pushback_vel += pushback;
        }
    }

    pub fn apply_fric(&mut self) {
        if self.grounded {
            self.apply_ground_fric()
        } else {
            self.apply_air_fric()
        }
    }

    pub fn apply_full_fric(&mut self, fric: FixedNum) {
        if self.vel.x.abs() > fric {
            self.vel.x -= self.vel.x * fric;
        }
    }

    pub fn apply_y_fric(&mut self, fric: FixedNum) {
        if self.vel.y.abs() > fric {
            self.vel.y -= self.vel.y * fric;
        }
    }

    pub fn apply_air_fric(&mut self) {
        // if self.vel.x.abs() > self.air_friction {
        self.vel.x -= (self.vel.x.abs() / self.max_air_speed) * self.air_friction * self.vel.x.sign();
        // } else {
            // self.vel.x *= 0;
        // }
    }

    pub fn apply_ground_fric(&mut self) {
        // if self.vel.x.abs() > self.ground_friction {
        //     self.vel.x -= self.vel.x * self.ground_friction;
        // } else {
        //     self.vel.x *= 0;
        // }
        self.vel.x -= (self.vel.x.abs() / self.max_ground_speed) * self.ground_friction * self.vel.x.sign()
    }

    pub fn apply_grav(&mut self) {
        if !self.grounded && self.vel.y < self.max_fall_speed {
            self.apply_force(FixedVec2::coords(FixedNum::from_num(0), self.gravity))
        }
    }

    pub fn apply_grav_custom(&mut self, gravity: FixedNum, fall_speed: FixedNum) {
        if !self.grounded && self.vel.y < fall_speed {
            self.apply_force(FixedVec2::coords(FixedNum::from_num(0), gravity))
        }
    }

    pub fn apply_forces(&mut self) {
        self.vel += self.accel;
        if self.grounded {
            if self.vel.x.abs() > self.max_ground_speed {
                self.vel.x = self.max_ground_speed * self.vel.x.sign();
            } 
        } else {
            if self.vel.x.abs() > self.max_air_speed {
                self.vel.x = self.max_air_speed * self.vel.x.sign();
            }
            if self.vel.y > self.max_fall_speed {
                self.vel.y = self.max_fall_speed;
            }
        }
        self.move_directly(self.vel);
        self.update_grounded();
        if self.grounded && self.vel.y > 0 {
            self.vel.y *= 0;
        }
        self.accel = FixedVec2::new();
    }

    pub fn apply_pushback(&mut self) {
        self.move_directly(self.get_relative_vec(FixedVec2::coords(self.pushback_vel, FixedNum::from_num(0))));
        if self.pushback_vel.abs() > PUSHBACK_FRIC {
            self.pushback_vel -= PUSHBACK_FRIC * self.pushback_vel.sign()
        } else {
            self.pushback_vel = FixedNum::from_num(0);
        }
    }

    pub fn apply_forces_no_limit(&mut self) {
        self.update_grounded();
        self.vel += self.accel;
        self.move_directly(self.vel);
        if self.grounded && self.vel.y > 0 {
            self.vel.y *= 0;
        }
        self.accel = FixedVec2::new();
    }

    pub fn reset_momentum(&mut self) {
        self.vel = FixedVec2::new();
        self.accel = FixedVec2::new();
    }


    pub fn update_grounded(&mut self) {
        self.set_grounded(self.pos.y >= 0)
    }

    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded
    }

    pub fn apply_force(&mut self, mut f: FixedVec2) {
        if self.grounded && f.y > 0 {
            f.y *= 0;
        }
        self.accel += f;
    }

    pub fn apply_force_relative(&mut self, f: FixedVec2) {
        self.apply_force(self.get_relative_vec(f))
    }

    pub fn get_relative_vec(&self, f: FixedVec2) -> FixedVec2 {
        match self.facing {
            Facing::Right => f,
            Facing::Left => f * FixedVec2::coords(-1, 1)
        }
    }

    pub fn get_position(&self) -> FixedVec2 {
        self.pos
    }

    pub fn set_position(&mut self, point: FixedVec2) {
        self.pos = point;
        if self.pos.y >= 0 {
            self.pos.y *= 0;
        };
        // self.pos.round()
    }
}