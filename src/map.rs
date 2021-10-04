use maplit::hashmap;
use std::collections::HashMap;
use std::fmt;

use crate::commands::Direction;
use crate::objects::Objects;

#[derive(Debug)]
pub struct Map {
    rooms: Vec<Room>,
}

impl Map {
    pub fn init() -> Self {
        Self {
            rooms: vec![
                Room {
                    id: Location::GameRoom,
                    name: String::from("Game room"),
                    description: String::from("There is a computer"),
                    exit: hashmap! {Direction::South => Door { where_to: Location::Kitchen, is_locked: false }},
                },
                Room {
                    id: Location::Kitchen,
                    name: String::from("Kitchen"),
                    description: String::from(
                        "There is a table with a key on it. A door leading north.",
                    ),
                    exit: hashmap! {Direction::North => Door { where_to: Location::GameRoom, is_locked: true }},
                },
            ],
        }
    }

    pub fn current_location_mut(&mut self, location: &Location) -> &mut Room {
        // ToDo - remove unwrap()
        self.rooms.iter_mut().find(|c| c.id == *location).unwrap()
    }

    pub fn current_location(&self, location: &Location) -> Room {
        // ToDo - remove unwrap()
        self.rooms
            .iter()
            .find(|c| c.id == *location)
            .unwrap()
            .clone()
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: Location,
    pub name: String,
    pub description: String,
    pub exit: HashMap<Direction, Door>,
}

#[derive(Debug, Clone, Copy)]
pub struct Door {
    pub where_to: Location,
    pub is_locked: bool,
}

impl Room {
    pub fn use_item(&mut self, item: Objects) -> bool {
        match item {
            Objects::Key if self.id == Location::Kitchen => {
                let exit = self.exit.get_mut(&Direction::North).unwrap();
                exit.is_locked = false;
                println!("The door is now open");
                true
            }
            Objects::Game if self.id == Location::GameRoom => {
                println!("I can't believe it. I am finally playing Monkey Island");
                true
            }
            _ => {
                println!("I can't do it");
                false
            }
        }
    }

    pub fn get_direction(&self, dir: &Direction) -> Option<Door> {
        self.exit.get(dir).copied()
    }

    pub fn change_description(&mut self, item: Objects) {
        if item == Objects::Key {
            self.description = String::from("There is a table. A door leading north.");
        }
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Location: {}\n{}",
            self.name.as_str(),
            self.description.as_str()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Location {
    GameRoom,
    Kitchen,
    Inventory,
    Dropped,
}
