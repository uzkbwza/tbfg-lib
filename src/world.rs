use gdnative::prelude::*;
use super::char::*;
use super::point::*;
use super::obj::*;

#[derive(ToVariant, FromVariant, Clone)]
pub struct WorldData {
    p1_data: CharacterData,
    p2_data: CharacterData,
}

pub struct GameWorld {
    stage_width: i32,
    char_distance: i32,
    p1: Option<GameCharacter>,
    p2: Option<GameCharacter>,
}

impl GameWorld {
    pub fn new(stage_width: i32, char_distance: i32) -> Self {
        let mut world = GameWorld {
            p1: None,
            p2: None,
            stage_width,
            char_distance,
        };
        world.init();
        world
    }

    pub fn init(&mut self) {
        let mut p1 = GameCharacter::new();
        let mut p2 = GameCharacter::new();
        
        p1.set_position(FixedVec2::coords(-self.char_distance, 0));
        p2.set_position(FixedVec2::coords(self.char_distance, 0));

        self.p1 = Some(p1);
        self.p2 = Some(p2);
    }

    pub fn get_player(&self, player_id: usize) -> &GameCharacter {
        match player_id {
            1 => self.p1.as_ref().unwrap(),
            2 => self.p2.as_ref().unwrap(),
            _ => panic!(),
        }
    }

    pub fn get_player_mut(&mut self, player_id: usize) -> &mut GameCharacter {
        match player_id {
            1 => self.p1.as_mut().unwrap(),
            2 => self.p2.as_mut().unwrap(),
            _ => panic!(),
        }
    }

    pub fn get_players(&self) -> [&GameCharacter; 2] {
        [self.p1.as_ref().unwrap(), self.p2.as_ref().unwrap()]
    }

    pub fn get_players_mut(&mut self) -> [&mut GameCharacter; 2] {
        [self.p1.as_mut().unwrap(), self.p2.as_mut().unwrap()]
    }

    pub fn tick(&mut self) {
        let data = self.get_data();
        for player in self.get_players_mut() {
            player.set_world_data(Some(data.clone()));
            player.tick()
        }
    }

    pub fn get_data(&self) -> WorldData {
        WorldData {
            p1_data: self.get_player(1).get_data(),
            p2_data: self.get_player(2).get_data(),
        }
    }
}