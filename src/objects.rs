use crate::map::Location;

pub struct Items {
    pub list: Vec<Item>,
}

impl Items {
    pub fn new() -> Self {
        Self {
            list: vec![
                Item {
                    id: Objects::Table,
                    name: String::from("Table"),
                    description: String::from("It's an old table"),
                    location: Location::Kitchen,
                    can_pick_up: false,
                },
                Item {
                    id: Objects::Key,
                    name: String::from("Key"),
                    description: String::from("It's a key"),
                    location: Location::Kitchen,
                    can_pick_up: true,
                },
                Item {
                    id: Objects::GameRoomDoor,
                    name: String::from("Door"),
                    description: String::from("It's a normal door"),
                    location: Location::Kitchen,
                    can_pick_up: false,
                },
                Item {
                    id: Objects::Computer,
                    name: String::from("Computer"),
                    description: String::from("It's a computer"),
                    location: Location::GameRoom,
                    can_pick_up: false,
                },
            ]
        }
    }

    pub fn get_item_description(&self, item: Objects, player_location: Location) -> &str {
        // return the item if player is in the correct room
        self.list
            .iter()
            .find(|c| c.id == item && c.location == player_location)
            .map(|c| c.description.as_str())
            .unwrap_or("Not found")
    }
}

#[derive(Debug)]
pub struct Item {
    id: Objects,
    name: String,
    pub description: String,
    location: Location,
    can_pick_up: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Objects {
    Key,
    Table,
    GameRoomDoor,
    Computer,
    NotFound,
}

impl Objects {
    pub fn get(obj: &str) -> Objects {
        match obj {
            "key" => Objects::Key,
            "table" => Objects::Table,
            "door" => Objects::GameRoomDoor,
            "computer" => Objects::Computer,
            _ => Objects::NotFound,
        }
    }
}