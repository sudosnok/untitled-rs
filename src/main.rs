mod rooms;
mod _types;
mod player;
mod monster;
#[allow(unused)]
use std::io::{self, Read};

use rooms::Dungeon;
use player::Player;


fn main(){

    let p = Player::new();
    let d = Dungeon::new(p);

    //println!("main : {}", d);

    d.player.check_bag();

    //println!("main : {:?}", d.room_map.keys());
    

    intro(d.current_room().desc.as_str());
    let _x = main_loop(d);
}

fn main_loop(mut d: Dungeon) -> io::Result<()>{
    let mut running = true;
    let mut buffer = String::new();
    'running: loop{
        match io::stdin().read_line(&mut buffer){
            Ok(_) => {
                let input = buffer.trim_end(); // remove the CRLF from the right side
                if input == "q"{    // if they want to stop
                    println!("Sorry to see you go.");
                    running = false;
                } else if input.starts_with("move ") { // if `move ` is at the start of the string, split and recollect them
                    let direction: Vec<&str> = input.split(' ').collect();
                    match direction[1].to_lowercase().as_str(){
                        n @ "north" => {
                            d.move_to(n);
                            d.print_paths();
                        }
                        e @ "east" => {
                            d.move_to(e);
                            d.print_paths();
                        }
                        s @ "south" => {
                            d.move_to(s);
                            d.print_paths();
                        },
                        w @ "west" => {
                            d.move_to(w);
                            d.print_paths();
                        }
                        other => {
                            println!("{:?} is not a valid direction.", other);
                            d.print_paths();
                        }
                    }
                    
                } else {
                    println!("{}", buffer)
                }
                buffer.clear();
            }
            Err(why) => println!("{}", why)
        }
        if running == false{
            break 'running
        }
    }
    Ok(())
}

fn intro(desc: &str){
    println!("You find yourself in a room, no recollection of who you were before.");
    println!("{}", desc);
    println!("`look around` to search the room for anything useful.");
    println!("`move <cardinal>` to proceed in that direction.");
}



fn _main_one(){
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

