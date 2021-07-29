use std::fmt::{self, Display};
use std::fs::File;
use std::io;

use rand::prelude::*;
use serde::Deserialize;




#[derive(Debug)]
#[derive(Deserialize)]
pub struct Item{pub name: String, pub id: u8, pub value: u8}
#[allow(dead_code)]
impl Item{
    pub fn new() -> Self{
        let name = String::from("Temporary Name");
        let id = 0u8;
        let value = 0u8;
        Item{name: name, id: id, value: value}
    }
}
#[allow(dead_code, unused_variables)]
impl Display for Item{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Item; name={} id={} value={}", self.name, self.id, self.value)
    }
}

#[derive(Debug)]
pub struct Container{items: Vec<Item>, rng: ThreadRng}
#[allow(dead_code)]
impl Container{
    pub fn new() -> io::Result<Self>{
        let container_trng = thread_rng();
        let file = File::open("./items.json")?;
        let items = serde_json::from_reader(file)?;
        Ok(Container{items: items, rng: container_trng})
    }

    pub fn get_one(&mut self) -> Item{
        let mut idx = self.rng.next_u32() as u8;
        if idx < 254{
            self.items.swap_remove(idx as usize)
        } else {
            idx /= self.rng.next_u32() as u8;
            self.items.swap_remove(idx as usize)
        }
    }
}