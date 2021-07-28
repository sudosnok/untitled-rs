use std::fmt::{self, Display};

use serde::Deserialize;
use serde_json::{self, from_str};
use rand::prelude::*;

use super::_types::{Item, Container};


fn pass(){}

#[derive(Deserialize, Debug)]
struct MStats{atk: u8, def: u8, hp: u16, max_hp: u16, xp_drop: u8}
impl MStats{
    pub fn default_monster() -> serde_json::Result<Self>{
        let data = r#"{
            "atk": 5,
            "def": 5,
            "hp": 15,
            "max_hp": 15,
            "xp_drop": 10
        }"#;
        let stat: MStats = from_str(data)?;
        Ok(stat)
    }
    pub fn default_zombie() -> serde_json::Result<Self>{
        let data = r#"{
            "atk": 7,
            "def": 2,
            "hp": 12,
            "max_hp": 12,
            "xp_drop": 15
        }"#;
        let stat: MStats = from_str(data)?;
        Ok(stat)
    }
}


#[derive(Debug)]
pub struct Monster{stats: MStats, mtype: String, loot: Vec<Item>}
#[allow(unused)]
impl Monster{
    pub fn monster() -> Self{
        let stats = match MStats::default_monster(){
            Ok(stats) => stats,
            Err(why) => panic!("Panicked at Monster::monster : {}", why)
        };
        let loot: Vec<Item> = Vec::with_capacity(1);
        Monster{ stats: stats, mtype: String::from("Monster"), loot: loot }
    }
    pub fn zombie() -> Self{
        let stats = match MStats::default_zombie(){
            Ok(stats) => stats,
            Err(why) => panic!("Panicked at Monster::zombie : {}", why)
        };
        let loot: Vec<Item> = Vec::with_capacity(1);
        Monster{ stats: stats, mtype: String::from("Zombie"), loot: loot }
    }
    pub fn update(&mut self, item: Option<Item>){
        match item{
            Some(item) => self.loot.push(item),
            _ => pass()
        };
    }
    pub fn display(&self){
        match (self.stats.hp / self.stats.max_hp) * 100u16{
            80..=100 => println!("Very healthy, maybe you should deal some damage to it!"),
            60..=79  => println!("A little hurt, but it isn't going to die any time soon."),
            40..=59  => println!("Pretty roughed up, but still fighting."),
            20..=39  => println!("Very injured, not going to hold on for much longer."),
            _        => println!("On death's door, one wrong look could finish them.")
        }
    }
    pub fn die(&mut self) -> (u8, Vec<Item>){
        if self.loot.len() != 0{
            let item = vec![self.loot.remove(0)];
            (self.stats.xp_drop, item)
        } else {
            let item = Vec::new();
            (self.stats.xp_drop, item)
        }
    }
    pub fn take_damage(&mut self, damage: u16){
        if damage >= self.stats.hp{
            let item = self.die();
        } else {
            self.stats.hp -= damage;
        }
        self.display();
    }
}
impl Display for Monster{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self.mtype.as_str(){
            "Monster" => write!(f, "A grizzled husk, looks hungry for you."),
            "Zombie" => write!(f, "A weathered zombie, stumbling but dangerous."),
            _ => write!(f, "Not sure whats happened here.")
        }
    }
}

#[derive(Debug)]
pub struct MonsterGenerator{monsters: Vec<Monster>, rng: ThreadRng, items: Container}
#[allow(unused)]
impl MonsterGenerator{
    pub fn new() -> Self{
        let monsters: Vec<Monster> = Vec::with_capacity(8);
        let rng = thread_rng();
        let container = match Container::new(){
            Ok(container) => container,
            Err(why) => panic!("Panicked at MonsterGenerator::new : {}", why)
        };
        MonsterGenerator{monsters: monsters, rng: rng, items: container}
    }
    pub fn get_loot(&mut self) -> Option<Item>{
        match self.rng.next_u32() % 3 {
            2 => Some(self.items.get_one()),
            _ => None
        }
    }
    pub fn generate(&mut self) -> Monster{
        let item = self.get_loot();
        let mut m = match self.rng.next_u32() % 2{
            0 => Monster::monster(),
            1 => Monster::zombie(),
            _ => panic!("Not possible.")
        };
        m.update(item);
        m
    }
}