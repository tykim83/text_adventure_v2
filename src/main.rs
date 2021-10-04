mod commands;
mod map;
mod objects;
mod player;

use commands::{Commands, LookAt};
use map::{Location, Map};
use objects::{Items, Objects};
use player::Player;

use std::io;
use std::io::Write;

fn main() {
    // Init game entities
    let mut player = Player::new(Location::Kitchen);
    let mut map = Map::init();
    let mut items = Items::new();

    // Run game
    run(&mut player, &mut map, &mut items);
}

fn run(player: &mut Player, map: &mut Map, items: &mut Items) {
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
                    Some(loc) if loc.is_locked => println!("It's locked"),
                    Some(loc) => {
                        player.change_location(loc.where_to);
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
                Objects::NotFound => println!("I can't see it"),
                _ => {
                    println!(
                        "{}",
                        items.pick_up_item(item, map.current_location_mut(&player.location))
                    );
                    room = map.current_location(&player.location);
                }
            },

            Commands::Use(item) => match item {
                _ if items.is_in_inventory(item) => {
                        if room.use_item(item) {
                            items.drop_item(item)
                        }  
                }
                _ => println!("I can't see it"),
            },

            Commands::Exit => break,

            Commands::None => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}
