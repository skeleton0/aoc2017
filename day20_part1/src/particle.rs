use std::cmp::Ordering;

use three_dimensional::ThreeDimensionalI32;

#[derive(Debug)]
pub struct Particle {
    acceleration: ThreeDimensionalI32,
    velocity: ThreeDimensionalI32,
    position: ThreeDimensionalI32,
}

impl Particle {
    pub fn new(acceleration: ThreeDimensionalI32, velocity: ThreeDimensionalI32, position: ThreeDimensionalI32) -> Particle {
        Particle {
            acceleration,
            velocity,
            position,
        }
    }

    pub fn long_term_proximity_to_origin(&self, other: &Particle) -> Ordering {
        let mut result = self.acceleration.cmp(&other.acceleration);

        if let Ordering::Equal = result {
            result = self.velocity.cmp(&other.velocity);

            if let Ordering::Equal = result {
                result = self.position.cmp(&other.position);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_term_proximity_to_origin_equal() {
        let a = Particle::new(ThreeDimensionalI32::new(5, -2, 7),
                              ThreeDimensionalI32::new(10, 5, -22),
                              ThreeDimensionalI32::new(-2, 7, 18));

        let b = Particle::new(ThreeDimensionalI32::new(3, -4, 7),
                              ThreeDimensionalI32::new(5, -5, 27),
                              ThreeDimensionalI32::new(1, -8, 18));

        assert_eq!(a.long_term_proximity_to_origin(&b), Ordering::Equal);
    }

    #[test]
    fn long_term_proximity_to_origin_greater() {
        let a = Particle::new(ThreeDimensionalI32::new(5, -5, 7),
                              ThreeDimensionalI32::new(10, 5, -22),
                              ThreeDimensionalI32::new(-2, 7, 18));

        let b = Particle::new(ThreeDimensionalI32::new(3, -4, 7),
                              ThreeDimensionalI32::new(5, -5, 27),
                              ThreeDimensionalI32::new(1, -8, 18));

        assert_eq!(a.long_term_proximity_to_origin(&b), Ordering::Greater);
    }

    #[test]
    fn long_term_proximity_to_origin_less() {
        let a = Particle::new(ThreeDimensionalI32::new(5, -2, 7),
                              ThreeDimensionalI32::new(10, 5, -22),
                              ThreeDimensionalI32::new(1, 7, 18));

        let b = Particle::new(ThreeDimensionalI32::new(3, -4, 7),
                              ThreeDimensionalI32::new(5, -5, 27),
                              ThreeDimensionalI32::new(1, -8, 18));

        assert_eq!(a.long_term_proximity_to_origin(&b), Ordering::Less);
    }
}