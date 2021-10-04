use crate::map::{Location, Room};

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
                    can_picked_up: false,
                },
                Item {
                    id: Objects::Key,
                    name: String::from("Key"),
                    description: String::from("It's a key"),
                    location: Location::Kitchen,
                    can_picked_up: true,
                },
                Item {
                    id: Objects::GameRoomDoor,
                    name: String::from("Door"),
                    description: String::from("It's a normal door"),
                    location: Location::Kitchen,
                    can_picked_up: false,
                },
                Item {
                    id: Objects::Computer,
                    name: String::from("Computer"),
                    description: String::from("It's a computer"),
                    location: Location::GameRoom,
                    can_picked_up: false,
                },
                Item {
                    id: Objects::Game,
                    name: String::from("Game"),
                    description: String::from(
                        "It's a copy of Monkey Island, can't wait to play it.",
                    ),
                    location: Location::Inventory,
                    can_picked_up: true,
                },
            ],
        }
    }

    pub fn get_item_description(&self, item: Objects, player_location: Location) -> &str {
        // return the item if player is in the correct room
        self.list
            .iter()
            .find(|c| {
                c.id == item && (c.location == player_location || c.location == Location::Inventory)
            })
            .map(|c| c.description.as_str())
            .unwrap_or("Not found")
    }

    pub fn is_game_over(&self) -> bool {
        self.list
            .iter()
            .any(|c| c.location == Location::Inventory)
    }

    pub fn inventory(&self) {
        for (index, item) in self
            .list
            .iter()
            .filter(|c| c.location == Location::Inventory)
            .enumerate()
        {
            println!("{}: {}", index + 1, item.name);
        }
    }

    pub fn is_in_inventory(&self, item: Objects) -> bool {
        self.list
            .iter()
            .any(|c| c.location == Location::Inventory && c.id == item)
    }

    pub fn pick_up_item(&mut self, item: Objects, room: &mut Room) -> String {
        let result = self
            .list
            .iter_mut()
            .find(|c| c.id == item && c.can_picked_up);

        match result {
            Some(i) if i.location == room.id => {
                room.change_description(item);
                i.location = Location::Inventory;
                format!("I have got the {}", i.name)
            }
            Some(_) => String::from("I cant see it"),
            None => String::from("I can't pick it up"),
        }
    }

    pub fn drop_item(&mut self, item: Objects) {
        let result = self
            .list
            .iter_mut()
            .find(|c| c.id == item && c.can_picked_up);

        if let Some(i) = result { i.location = Location::Dropped }
    }
}

#[derive(Debug)]
pub struct Item {
    id: Objects,
    name: String,
    pub description: String,
    location: Location,
    can_picked_up: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Objects {
    Key,
    Table,
    GameRoomDoor,
    Computer,
    Game,
    NotFound,
}

impl Objects {
    pub fn get(obj: &str) -> Objects {
        match obj {
            "key" => Objects::Key,
            "table" => Objects::Table,
            "door" => Objects::GameRoomDoor,
            "computer" => Objects::Computer,
            "game" => Objects::Game,
            _ => Objects::NotFound,
        }
    }
}
