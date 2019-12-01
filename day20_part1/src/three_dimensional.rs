use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct ThreeDimensionalI32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PartialOrd for ThreeDimensionalI32 {
    fn partial_cmp(&self, other: &ThreeDimensionalI32) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ThreeDimensionalI32 {
    fn cmp(&self, other: &ThreeDimensionalI32) -> Ordering {
        let self_abs = self.x.abs() + self.y.abs() + self.z.abs();
        let other_abs = other.x.abs() + other.y.abs() + other.z.abs();

        self_abs.cmp(&other_abs)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_function_works() {
        let a = ThreeDimensionalI32::new(5, -3, 22);

        let b = ThreeDimensionalI32 {
            x: 5,
            y: -3,
            z: 22,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn is_equal() {
        let a = ThreeDimensionalI32::new(5, -3, 22);
        let b = ThreeDimensionalI32::new(5, -3, 22);

        assert_eq!(a, b);
    }

    #[test]
    fn not_equal() {
        let a = ThreeDimensionalI32::new(5, -3, 22);
        let b = ThreeDimensionalI32::new(5, -10, 22);

        assert_ne!(a, b);
    }

    #[test]
    fn greater_than() {
        let a = ThreeDimensionalI32::new(5, -3, 22);
        let b = ThreeDimensionalI32::new(5, -10, 22);

        assert!(b > a);
    }

    #[test]
    fn less_than() {
        let a = ThreeDimensionalI32::new(5, -3, 22);
        let b = ThreeDimensionalI32::new(-2, 20, -22);

        assert!(a < b);
    }
}