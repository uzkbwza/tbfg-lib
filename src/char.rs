use gdnative::prelude::*;
use super::obj::*;
use super::point::*;
use super::utils::*;
use super::world::*;

#[derive(ToVariant, FromVariant, Clone)]
pub struct CharacterData {
    point: Vector2,
}

impl ObjectData for CharacterData {}

pub struct GameCharacter {
    point: FixedVec2,
    world_data: Option<WorldData>
}

impl GameCharacter {
    pub fn new() -> Self {
        GameCharacter {
            point: FixedVec2::new(),
            world_data: None,
        }
    }

    pub fn set_world_data(&mut self, world_data: Option<WorldData>) {
        self.world_data = world_data
    }
}

impl GameObject<CharacterData> for GameCharacter {
    fn tick(&mut self) {
        self.point.x += FixedNum::from_num(1) / 5;
        // println!("{:?}", self.point.x)
    }

    fn get_data(&self) -> CharacterData {
        CharacterData {
            point: self.point.into()
        }
    }

    fn get_position(&self) -> FixedVec2 {
        self.point
    }

    fn set_position(&mut self, point: FixedVec2) {
        self.point = point;
    }
}