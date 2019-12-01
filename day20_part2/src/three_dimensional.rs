#[derive(Debug)]
pub struct ThreeDimensionalI32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ThreeDimensionalI32 {
    pub fn new(x: i32, y: i32, z: i32) -> ThreeDimensionalI32 {
        ThreeDimensionalI32 {
            x,
            y,
            z
        }
    }
}