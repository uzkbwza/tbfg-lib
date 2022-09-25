use gdnative::prelude::*;

pub mod char;
pub mod world;
pub mod obj;
pub mod utils;
pub mod point;

use world::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameSimulation {
    world: Option<GameWorld>,
    #[property(default=1000)]
    stage_width: i32,
    #[property(default=200)]
    char_distance: i32,
    #[property(no_editor)]
    is_ready: bool,
    current_tick: i32,
    #[property]
    frames: Vec<WorldData>,
}

#[methods]
impl GameSimulation {
    fn new(_base: &Node) -> Self {
        GameSimulation {
            world: None,
            stage_width: 1000,
            char_distance: 200,
            is_ready: false,
            current_tick: -1,
            frames: Vec::new()
        }
    }

    #[method]
    fn _ready(&mut self, #[base] _base: &Node) {
        self.init()
    }

    #[method]
    fn init(&mut self) {
        let mut world = GameWorld::new(self.stage_width, self.char_distance);
        world.init();
        self.world = Some(world);
        self.is_ready = true;
    }
    
    #[method]
    fn tick(&mut self) {
        if let Some(world) = &mut self.world {
            world.tick();
            self.current_tick += 1;
            self.frames.push(world.get_data().clone())
        }
    }
    
    fn get_world(&self) -> &GameWorld {
        self.world.as_ref().unwrap()
    }

    #[method]
    pub fn get_all_frames(&self) -> &Vec<WorldData> {
        &self.frames
    }

    #[method]
    fn get_simulation_data(&self) -> WorldData {
        self.get_world().get_data()
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<GameSimulation>();
}

godot_init!(init);

#[cfg(test)]
mod tests {
    use super::point::*;
    use super::utils::*;
    #[test]
    fn point_arith() {
        let p = FixedVec2::coords(1, 1);
        assert_eq!(p + FixedVec2::coords(1, 1), FixedVec2::coords(2, 2));
        assert_eq!(p - FixedVec2::coords(1, 1), FixedVec2::coords(0, 0));
        assert_eq!(p * 5, FixedVec2::coords(5, 5));
        assert_eq!(p / 5, FixedVec2::coords(FixedNum::from_num(1) / 5, FixedNum::from_num(1) / 5));
    }
}