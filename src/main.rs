mod commands;
mod map;
mod objects;
mod player;

use commands::{Commands, LookAt};
use map::{Location, Map};
use objects::Items;
use player::Player;

use std::io;
use std::io::Write;

fn main() {
    // Init game entities
    let mut player = Player::new(Location::Kitchen);
    let map = Map::init();
    let mut items = Items::new();

    // Run game
    run(&mut player, &map, &mut items);
}

fn run(player: &mut Player, map: &Map, items: &mut Items) {
    // clear console
    print!("\x1B[2J\x1B[1;1H");
    let mut input = String::new();
    let mut room = map.current_location(&player.location);
    println!("{}", room);

    loop {
        input.clear();

        // Get user input
        print!("Enter: ");
        io::stdout().flush().expect("flush failed!");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match Commands::parse(&input.trim().to_lowercase()) {
            Commands::Move(dir) => {
                match room.get_direction(&dir) {
                    Some(loc) => {
                        player.change_location(loc);
                        room = map.current_location(&player.location);
                        print!("\x1B[2J\x1B[1;1H");
                        println!("{}", room);
                    }
                    None => println!("Wrong direction!"),
                };
            }

            Commands::Look(item) => match item {
                LookAt::Room => {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("{}", room);
                }
                LookAt::Item(item) => {
                    println!("{}", items.get_item_description(item, player.location))
                }
                LookAt::Inventory => items.inventory(),
            },

            Commands::Get(item) => match item {
                objects::Objects::NotFound => println!("I can't see it"),
                _ => println!("{}", items.pick_up_item(item, player.location)),
            },

            Commands::Exit => break,

            Commands::None => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}

// ToDo - V2
// Only look at items located in the current room - Done
// look at inventory - check inventory when look at item - Done
// get item - Done
// Fix room description when item removed
// open door - this will work if correct item is in the inventory

// Todo - V3
// use item
// 5 rooms
// use rust traits
// dynamic storage -- MongoDB???
// players items

// V4 Rocket and Yew

// V5 Bevy
