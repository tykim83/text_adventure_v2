use crate::map::Location;

#[derive(Debug)]
pub struct Player {
    pub location: Location,
}

impl Player {
    pub fn new(location: Location) -> Player {
        Player { location: location }
    }

    pub fn change_location(&mut self, new_location: Location) {
        self.location = new_location
    }
}
