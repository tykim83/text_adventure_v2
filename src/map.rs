use std::collections::HashMap;
use std::fmt;
use maplit::hashmap;

use crate::commands::Direction;

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
                    exit: hashmap!{Direction::South => Location::Kitchen},
                },
                Room {
                    id: Location::Kitchen,
                    name: String::from("Kitchen"),
                    description: String::from(
                        "There is a table with a key on it. A door leading north.",
                    ),
                    exit: hashmap!{Direction::North => Location::GameRoom},
                },
            ],
        }
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
    id: Location,
    pub name: String,
    pub description: String,
    pub exit: HashMap<Direction, Location>,
}

impl Room {
    pub fn get_direction(&self, dir: &Direction) -> Option<Location> {
        self.exit.get(dir).copied()
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location: {}\n{}", self.name.as_str(), self.description.as_str())
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Location {
    GameRoom,
    Kitchen,
    Inventory,
}
