use gdnative::prelude::*;
use super::obj::*;
use super::point::*;
use super::utils::*;

#[derive(ToVariant, FromVariant, Clone)]
pub struct FGObjectData {
    object_data: BaseObjectData,
}

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct FGObject {
    obj: BaseObject,
    #[property(default=1)]
    id: i64, // 1 is p1, 2 is p2
}

#[methods]
impl FGObject {
    pub fn new(_base: &Reference) -> Self {
        FGObject {
            obj: BaseObject::new(),
            id: 1,
        }
    }

    #[method]
    pub fn copy_to(&self, #[base] _base: TRef<Reference>, instance: Instance<FGObject, Shared>) {
        instance.map_mut(|p: &mut FGObject, _base: TRef<Reference, Shared>| {
            p.obj = self.obj.clone();
            p.id = self.id;
        });
    }

    // #[method]
    // pub fn copy_to(&self, #[base] _base: TRef<Reference>, instance: Variant) {
    //     let instance = {
    //         instance
    //             .try_to_object::<Reference>()
    //             .unwrap()
    //     };

    //     let instance = unsafe {
    //         instance.assume_safe()
    //     };

    //     let instance = instance
    //         .cast_instance::<FGObject>()
    //         .unwrap();
        
    //     instance.map_mut(|p: &mut FGObject, _base| {
    //         p.obj = self.obj.clone();
    //         p.id = self.id;
    //     }).unwrap();
    //     // ...
    // }

    #[method]
    pub fn set_gravity(&mut self, gravity: String) {
        self.obj.gravity = FixedNum::unwrapped_from_str(&gravity)
    }

    #[method]
    pub fn set_ground_friction(&mut self, friction: String) {
        self.obj.ground_friction = FixedNum::unwrapped_from_str(&friction)
    }

    #[method]
    pub fn set_air_friction(&mut self, friction: String) {
        self.obj.air_friction = FixedNum::unwrapped_from_str(&friction)
    }

    #[method]
    pub fn set_max_ground_speed(&mut self, speed: String) {
        self.obj.max_ground_speed = FixedNum::unwrapped_from_str(&speed)
    }

    #[method]
    pub fn set_max_air_speed(&mut self, speed: String) {
        self.obj.max_air_speed = FixedNum::unwrapped_from_str(&speed)
    }

    #[method]
    pub fn set_max_fall_speed(&mut self, speed: String) {
        self.obj.max_fall_speed = FixedNum::unwrapped_from_str(&speed)
    }

    #[method]
    pub fn get_facing(&self) -> Facing {
        self.obj.facing
    }

    #[method]
    pub fn get_x_vel_int(&self) -> i32 {
        self.obj.vel.x.round().to_num()
    }
    
    #[method]
    pub fn add_pushback(&mut self, pushback: String) {
        self.obj.add_pushback(FixedNum::unwrapped_from_str(&pushback))
    }

    #[method]
    pub fn apply_pushback(&mut self) {
        self.obj.apply_pushback();
    }

    #[method]
    pub fn set_vel(&mut self, x: String, y: String) {
        self.obj.set_vel(FixedVec2::from_string(x, y));
    }

    // #[method]
    // pub fn _ready(&mut self) {
    //     self.init()
    // }

    // #[method]
    // pub fn init(&mut self) {
    //     if self.id != 1 {
    //         self.obj.facing = Facing::Left;
    //     }
    // }

    #[method]
    pub fn apply_forces(&mut self) {
        self.obj.apply_forces()
    }

    #[method]
    pub fn apply_forces_no_limit(&mut self) {
        self.obj.apply_forces_no_limit()
    }
    #[method]
    pub fn apply_force_str(&mut self, x: String, y: String) {
        self.obj.apply_force(
            FixedVec2::from_string(x, y)
        )
    }

    #[method]
    pub fn apply_force(&mut self, x: i32, y: i32) {
        self.obj.apply_force(
            FixedVec2::coords(x, y),
        )
    }
    
    #[method]
    pub fn set_facing(&mut self, facing: i32) {
        self.obj.set_facing_int(facing);
    }

    #[method]
    pub fn apply_force_relative_str(&mut self, x: String, y: String) {
        self.obj.apply_force_relative(
            FixedVec2::from_string(x, y)
        )
    }

    #[method]
    pub fn apply_force_relative(&mut self, x: i32, y: i32) {
        self.obj.apply_force_relative(
            FixedVec2::coords(x, y)
        )
    }

    #[method]
    pub fn apply_full_fric(&mut self, fric: String) {
        self.obj.apply_full_fric(FixedNum::from_str(&fric).unwrap())
    }

    #[method]
    pub fn apply_grav(&mut self) {
        self.obj.apply_grav()
    }

    #[method]
    pub fn apply_fric(&mut self) {
        self.obj.apply_fric()
    }

    #[method]
    pub fn get_data(&self) -> FGObjectData {
        FGObjectData { 
            object_data: self.obj.get_data()
        }
    }

    #[method]
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.obj.pos = FixedVec2::coords(x, y)
    }
    
    #[method]
    pub fn set_x(&mut self, x: i32) {
        self.obj.pos.x = FixedNum::from_num(x);
    }

    #[method]
    pub fn set_position_str(&mut self, x: String, y: String) {
        self.obj.pos = FixedVec2::from_string(x, y)
    }

    #[method]
    pub fn move_directly(&mut self, x: i32, y: i32) {
        self.obj.move_directly(FixedVec2::coords(x, y))
    }

    #[method]
    pub fn move_directly_relative(&mut self, x: i32, y: i32) {
        self.obj.move_directly_relative(FixedVec2::coords(x, y))
    }

    #[method]
    pub fn move_directly_relative_str(&mut self, x: String, y: String) {
        self.obj.move_directly_relative(FixedVec2::from_string(x, y))
    }

    #[method]
    pub fn move_directly_str(&mut self, x: String, y: String) {
        self.obj.move_directly(FixedVec2::from_string(x, y))
    }

    #[method]
    pub fn set_grounded(&mut self, grounded: bool) {
        self.obj.grounded = grounded
    }

    #[method]
    pub fn is_grounded(&self) -> bool {
        self.obj.grounded
    }

    #[method]
    pub fn update_grounded(&mut self) {
        self.obj.update_grounded()
    }

    #[method]
    pub fn reset_momentum(&mut self) {
        self.obj.reset_momentum()
    }
}