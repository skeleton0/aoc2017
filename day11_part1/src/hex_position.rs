use std::cmp;

pub enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

pub struct HexPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl HexPosition {
    pub fn new() -> HexPosition {
        HexPosition {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn move_position(&mut self, direction: HexDirection) {
        match direction {
            HexDirection::North => {
                self.x -= 1;
                self.y += 1;
            },
            HexDirection::NorthEast => {
                self.y += 1;
                self.z -= 1;
            },
            HexDirection::SouthEast => {
                self.x += 1;
                self.z -= 1;
            },
            HexDirection::South => {
                self.x += 1;
                self.y -= 1;
            },
            HexDirection::SouthWest => {
                self.y -= 1;
                self.z += 1;
            },
            HexDirection::NorthWest => {
                self.x -= 1;
                self.z += 1;
            }
        }
    }

    pub fn distance_from(&self, origin: &HexPosition) -> i32 {
        cmp::max(cmp::max(self.x.abs() - origin.x.abs(), self.y.abs() - origin.y.abs()), self.z.abs() - origin.z.abs())
    }
}