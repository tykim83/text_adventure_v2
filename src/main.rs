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
                objects::Objects::NotFound => println!("I can't see it"),
                _ => {
                    println!(
                        "{}",
                        items.pick_up_item(item, map.current_location_mut(&player.location))
                    );
                    room = map.current_location(&player.location);
                }
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
// Fix room description when item removed - Done
// locked door - Done
// use item
// use key(where there is the correct locked door)-> Open door
// use game(where there is a computer) -> will play the game -> Game Over

// Todo - V3
// 4 rooms
// rooms and player hold items
// use rust traits - ECS
// dynamic storage - MongoDB json
// use RefCell

// Room description sample
// There is an old table
// The key lie there next to a note.
// A door leading north

// V4 Rocket and Yew

// V5 Bevy
