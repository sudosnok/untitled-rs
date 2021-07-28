use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufReader, BufRead};

use rand::prelude::*;

use super::_types::{Item, Container};
use super::player::{Player};
use super::monster::{MonsterGenerator, Monster};



fn read_lines(filename: String) -> io::Result<Vec<String>>{
    let f = File::open(filename)?;
    let f = BufReader::new(f);
    let mut desc: Vec<String> = Vec::new();
    for line in f.lines(){
        if let Ok(line) = line{
            desc.push(line);
        }
    }
    Ok(desc)
}

fn do_nothing(){}


#[derive(Debug)]
pub struct Room{
    pub id: u16,
    pub desc: String,
    pub items: Vec<Item>,
    pub monsters: Vec<Monster>
}
#[allow(dead_code)]
impl Room{
    pub fn new() -> Self{
        let id = 0u16;
        let desc = String::from("Temporary description.");
        let items: Vec<Item> = Vec::new();
        let monsters: Vec<Monster> = Vec::with_capacity(1);
        Room{ id: id, desc: desc, items: items, monsters: monsters}
    }
    pub fn update(&mut self, item: Item){
        self.items.push(item);
    }
    pub fn add_monster(&mut self, monster: Monster){
        self.monsters.push(monster);
    }
}
impl Display for Room{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        if self.monsters.len() == 0{
            write!(f, "{}", self.desc)
        } else {
            let m = match self.monsters.get(1){
                Some(m) => m,
                None => panic!("Shouldn't be possible.")
            };
            write!(f, "{}, {}", self.desc, m)
        }
    }
}

#[derive(Debug)]
pub struct Dungeon{
    pub counter: u16, 
    pub descriptions: Vec<String>, 
    pub rooms: Vec<Room>, 
    pub rng: ThreadRng,
    pub container: Container,
    pub player: Player,
    pub m_generator: MonsterGenerator,
}
#[allow(dead_code)]
impl Dungeon{
    pub fn new(player: Player) -> Self{
        let counter = 0u16;
        let desc: Vec<String> = Vec::with_capacity(850);
        let rooms: Vec<Room> = Vec::with_capacity(8);
        let dungeon_trng = thread_rng();
        let container = match Container::new(){
            Ok(container) => container,
            Err(why) => panic!("{:?}", why),
        };
        let m_generator = MonsterGenerator::new();
        let mut d = Dungeon{ 
            counter: counter,
            descriptions: desc,
            rooms: rooms,
            rng: dungeon_trng,
            container: container,
            player: player,
            m_generator: m_generator,
        };
        d._load_lines();
        d.make_room();
        d._fill_rooms();
        d
    }
    fn _get_next_id(&mut self) -> u16{
        self.counter += 1;
        self.counter
    }

    fn _fill_rooms(&mut self){
        for room in self.rooms.iter_mut(){
            if self.rng.next_u32() % 3 == 0{
                let item = self.container.get_one();
                room.update(item);
            }
        }
    }

    fn _get_next_desc(&self) -> String{
        let desc = match self.descriptions.get(self.counter as usize){
            Some(desc) => desc,
            _ => "Failed!",
        };
        String::from(desc)
    }

    fn _load_lines(&mut self){
        let lines = match read_lines("./descriptions.txt".to_owned()){
            Ok(lines) => lines,
            _ => vec![String::new()]
        };
        for line in lines{
            self.descriptions.push(line);
        }
    }

    pub fn current_room(&self) -> &Room{
        if let Some(r) = self.rooms.get(0){
            r
        } else {
            panic!("No rooms set.")
        }
    }

    pub fn make_room(&mut self){
        let id = self._get_next_id();
        let desc = self._get_next_desc();
        let items: Vec<Item> = Vec::new();
        let monsters: Vec<Monster> = Vec::with_capacity(1);
        let mut room = Room{id: id, desc: desc, items:items, monsters: monsters};
        let monster = self.m_generator.generate();
        match self.rng.next_u32() % 2{
            1 => room.add_monster(monster),
            _ => do_nothing(),
        };
        self.rooms.push(room);
    }
}
impl Display for Dungeon{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Dungeon; room count:{}", self.counter)
    }
}
