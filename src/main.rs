mod rooms;
mod _types;
mod player;
mod monster;

use rooms::Dungeon;
use player::Player;



fn main(){
    let p = Player::new();
    let d = Dungeon::new(p);
    println!("{}", d);
    println!("{}", 254%3);

    d.player.check_bag();
    println!("{:?}", d.current_room());

    let mut v: Vec<u8> = vec![];
    println!("{:?}",v);
    let item = v.remove(0);
    println!("{:?}, {}", v, item);
}
