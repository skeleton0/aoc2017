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

    //returns time of collision (if one was found)
    fn linear_collision_check(&self, b: &Particle) -> Option<u32> {
        let a_t_coeff = self.velocity.x;
        let a_const = self.position.x;

        let b_t_coeff = b.velocity.x;
        let b_const = b.position.x;

        let c_t_coeff = a_t_coeff - b_t_coeff;
        let c_const = a_const - b_const;

        //if parallel lines
        if c_t_coeff == 0 {
            //if these functions are equivalent
            if a_const == b_const {
                return Some(0);
            } else {
                return None;
            }
        }

        //if time is not an integer (but instead has a decimal part)
        if ((c_const * -1) % c_t_coeff) != 0 {
            return None;
        }

        let time = (c_const * -1) / c_t_coeff;

        //if the collision happens before time began
        if time < 0 {
            return None;
        }

        //calculate positions of particles at time point
        let a_y_pos = self.velocity.y * time + self.position.y;
        let a_z_pos = self.velocity.z * time + self.position.z;
        let b_y_pos = b.velocity.y * time + b.position.y;
        let b_z_pos = b.velocity.z * time + b.position.z;

        //if there are collisions for both other dimensions at time point
        if a_y_pos == b_y_pos && a_z_pos == b_z_pos {
            Some(time as u32)
        } else {
            None
        }
    }

    //returns time of collision (if one was found)
    fn quadratic_collision_check(&self, b: &Particle) -> Option<(u32, u32)> {
        let a_sqr_t_coeff = self.acceleration.x as f32 / 2.0;
        let a_t_coeff = a_sqr_t_coeff + self.velocity.x as f32;
        let a_const = self.position.x;

        let b_sqr_t_coeff = b.acceleration.x as f32 / 2.0;
        let b_t_coeff = b_sqr_t_coeff + b.velocity.x as f32;
        let b_const = b.position.x;

        let c_sqr_t_coeff = a_sqr_t_coeff - b_sqr_t_coeff;
        let c_t_coeff = a_t_coeff - b_t_coeff;
        let c_const = a_const - b_const;

        //if the resulting equation is linear
        if c_sqr_t_coeff == 0.0 {
           if c_t_coeff == 0.0 {
               return None;
           }

           //if time is not an integer (but instead has a decimal part)
           if ((c_const * -1) as f32 % c_t_coeff) != 0.0 {
               return None;
           }
   
           let time = (c_const * -1) as f32 / c_t_coeff;
   
           //if the collision happens before time began
           if time < 0.0 {
               return None;
           }

           //check if other dimensions collide
           let a_pos_y = (self.acceleration.y as f32 / 2.0) * time.powf(2.0) + ((self.acceleration.y as f32 / 2.0) + self.velocity.y as f32) * time + self.position.y as f32;
           let a_pos_z = (self.acceleration.z as f32 / 2.0) * time.powf(2.0) + ((self.acceleration.z as f32 / 2.0) + self.velocity.z as f32) * time + self.position.z as f32;
           let b_pos_y = (b.acceleration.y as f32 / 2.0) * time.powf(2.0) + ((b.acceleration.y as f32 / 2.0) + b.velocity.y as f32) * time + b.position.y as f32;
           let b_pos_z = (b.acceleration.z as f32 / 2.0) * time.powf(2.0) + ((b.acceleration.z as f32 / 2.0) + b.velocity.z as f32) * time + b.position.z as f32;   
           
           if a_pos_y == b_pos_y || a_pos_z == b_pos_z {
               return Some((time as u32, time as u32));
           } else {
               return None;
           }
           
        }

        let descriminant = (c_t_coeff * -1.0).powf(2.0) - 4.0 * c_sqr_t_coeff * c_const as f32;

        //if no real roots are found
        if descriminant < 0.0 {
            return None;
        }
        
        let time1 = ((c_t_coeff * -1.0) + descriminant.sqrt()) / (2.0 * c_sqr_t_coeff);
        let time2 = ((c_t_coeff * -1.0) - descriminant.sqrt()) / (2.0 * c_sqr_t_coeff);

        let mut time1_is_valid = false;
        let mut time2_is_valid = false;

        if time1 >= 0.0 && time1 % 1.0 == 0.0 {
            time1_is_valid = true;
        }

        if time2 >= 0.0 && time2 % 1.0 == 0.0 {
            time2_is_valid = true;
        }

        //check if other dimensions collide at this time
        if time1_is_valid {
            let a_pos_y = (self.acceleration.y as f32 / 2.0) * time1.powf(2.0) + ((self.acceleration.y as f32 / 2.0) + self.velocity.y as f32) * time1 + self.position.y as f32;
            let a_pos_z = (self.acceleration.z as f32 / 2.0) * time1.powf(2.0) + ((self.acceleration.z as f32 / 2.0) + self.velocity.z as f32) * time1 + self.position.z as f32;
            let b_pos_y = (b.acceleration.y as f32 / 2.0) * time1.powf(2.0) + ((b.acceleration.y as f32 / 2.0) + b.velocity.y as f32) * time1 + b.position.y as f32;
            let b_pos_z = (b.acceleration.z as f32 / 2.0) * time1.powf(2.0) + ((b.acceleration.z as f32 / 2.0) + b.velocity.z as f32) * time1 + b.position.z as f32;

            if a_pos_y != b_pos_y || a_pos_z != b_pos_z {
                time1_is_valid = false;
            }
        }

        if time2_is_valid {
            let a_pos_y = (self.acceleration.y as f32 / 2.0) * time2.powf(2.0) + ((self.acceleration.y as f32 / 2.0) + self.velocity.y as f32) * time2 + self.position.y as f32;
            let a_pos_z = (self.acceleration.z as f32 / 2.0) * time2.powf(2.0) + ((self.acceleration.z as f32 / 2.0) + self.velocity.z as f32) * time2 + self.position.z as f32;
            let b_pos_y = (b.acceleration.y as f32 / 2.0) * time2.powf(2.0) + ((b.acceleration.y as f32 / 2.0) + b.velocity.y as f32) * time2 + b.position.y as f32;
            let b_pos_z = (b.acceleration.z as f32 / 2.0) * time2.powf(2.0) + ((b.acceleration.z as f32 / 2.0) + b.velocity.z as f32) * time2 + b.position.z as f32;

            if a_pos_y != b_pos_y || a_pos_z != b_pos_z {
                time2_is_valid = false;
            }
        }

        if time1_is_valid && time2_is_valid {
            Some((time1 as u32, time2 as u32))
        }
        else if time1_is_valid {
            let time1 = time1 as u32;
            Some((time1, time1))
        }
        else if time2_is_valid {
            let time2 = time2 as u32;
            Some((time2, time2))
        }
        else {
            None
        }
    }

    pub fn collision_check(&self, b: &Particle) -> Option<(u32, u32)> {
        if self.acceleration.x == 0 && b.acceleration.x == 0 {
            let time = self.linear_collision_check(b);
            
            match time {
                Some(val) => Some((val, val)),
                None => None,
            }
        } else {
            self.quadratic_collision_check(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_collision_check_with_collision() {
        let a = Particle::new(ThreeDimensionalI32::new(0, 0, 0), 
                              ThreeDimensionalI32::new(3, 5, -3), 
                              ThreeDimensionalI32::new(-6, 2, -9));

        let b = Particle::new(ThreeDimensionalI32::new(0, 0, 0), 
                              ThreeDimensionalI32::new(1, 2, 4), 
                              ThreeDimensionalI32::new(-2, 8, -23));

        assert_eq!(a.collision_check(&b), Some((2, 2)));
    }

    #[test]
    fn linear_collision_check_with_no_collision() {
        let a = Particle::new(ThreeDimensionalI32::new(0, 0, 0), 
                              ThreeDimensionalI32::new(3, 5, -3), 
                              ThreeDimensionalI32::new(-6, 2, 9));

        let b = Particle::new(ThreeDimensionalI32::new(0, 0, 0), 
                              ThreeDimensionalI32::new(1, 2, 4), 
                              ThreeDimensionalI32::new(-2, 8, 23));

        assert_eq!(a.collision_check(&b), None);
    }

    #[test]
    fn linear_collision_check_with_parallel_paths() {
        let a = Particle::new(ThreeDimensionalI32::new(0, 0, 0), 
                              ThreeDimensionalI32::new(3, 5, 4), 
                              ThreeDimensionalI32::new(-6, 2, -9));

        let b = Particle::new(ThreeDimensionalI32::new(0, 0, 0), 
                              ThreeDimensionalI32::new(1, 2, 4), 
                              ThreeDimensionalI32::new(-2, 8, -23));

        assert_eq!(a.collision_check(&b), None);
    }
}