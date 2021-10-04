use crate::objects::Objects;

#[derive(Debug)]
pub enum Commands {
    Move(Direction),
    Look(LookAt),
    Get(Objects),
    Use(Objects),
    Exit,
    None,
}

impl Commands {
    pub fn parse(cmd: &str) -> Commands {
        let mut v = cmd.split(|c| c == ' ');

        let cmd = v.next().unwrap_or("");
        let item = v.next();

        match cmd {
            "exit" | "q" => Commands::Exit,
            "north" | "n" => Commands::Move(Direction::North),
            "south" | "s" => Commands::Move(Direction::South),
            "west" | "w" => Commands::Move(Direction::West),
            "east" | "e" => Commands::Move(Direction::East),
            "look" => match item {
                Some(c) => match c {
                    "inventory" => Commands::Look(LookAt::Inventory),
                    _ => Commands::Look(LookAt::Item(Objects::get(c))),
                },
                None => Commands::Look(LookAt::Room),
            },
            "get" => match item {
                Some(c) => Commands::Get(Objects::get(c)),
                None => Commands::Get(Objects::NotFound),
            },
            "use" => match item {
                Some(c) => Commands::Use(Objects::get(c)),
                None => Commands::Use(Objects::NotFound),
            },
            _ => Commands::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum LookAt {
    Item(Objects),
    Inventory,
    Room,
}
