use std::fmt::{self, Display};

use serde::Deserialize;
use serde_json;
use rand::prelude::*;

use super::_types::{Item};


#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct PlayerStats{
    atk: u8, 
    def: u8, 
    hp: u16, 
    max_weight: u16, 
    xp_remaining: u16,
    level: u8,
}
#[allow(unused)]
impl PlayerStats{
    fn default_player() -> serde_json::Result<Self>{
        let data = r#"{
            "atk": 5,
            "def": 5,
            "hp": 50,
            "max_weight": 70,
            "xp_remaining": 45,
            "level": 1
        }"#;
        let stat: PlayerStats = match serde_json::from_str(data){
            Ok(stat) => stat,
            Err(why) => panic!("Panicked at PlayerStats::default_player : {}", why)
        };
        Ok(stat)
    }
}


#[derive(Debug)]
#[allow(unused)]
pub struct Player{name: String, pub stats: PlayerStats, pub inventory: Vec<Item>, rng: ThreadRng}
#[allow(unused)]
impl Player{
    pub fn new() -> Self{
        let name = String::from("Bob the Explorer");
        let stats = match PlayerStats::default_player(){
            Ok(stats) => stats,
            Err(why) => panic!("Panicked in Player::new : {}", why),
        };
        let inventory: Vec<Item> = Vec::with_capacity(255);
        let player_rng = thread_rng();
        Player{name: name, stats: stats, inventory: inventory, rng: player_rng}
    }
    pub fn take(&mut self, item: Item){
        self.inventory.push(item);
    }
    pub fn attack(&mut self) -> u8{
        let modifier = self.rng.gen_range(1..3);
        let lo = self.stats.atk - modifier;
        let hi = self.stats.atk + modifier;
        self.rng.gen_range(lo..hi)
    }
    pub fn check_bag(&self){
        if self.inventory.len() != 0{
            print!("Items;");
            for item in self.inventory.iter(){
                println!("{}", *item);
            }
        } else {
            println!("Empty bag!");
        }
    }
}
impl Display for Player{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}, {:?}", self.name, self.stats)
    }
}

