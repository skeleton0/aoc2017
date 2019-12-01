#[derive(Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self) 
    }
}

