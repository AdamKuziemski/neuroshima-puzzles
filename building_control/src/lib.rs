use serde::Deserialize;

#[derive(Deserialize)]
pub struct Building {
    pub name: String,
}

pub enum BuildingError {
    // todo
}

impl Building {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn from_file(file: String) -> Self {
        Self {
            name: String::from("New building!")
        }
    }
}
