const DENOMINATOR: u64 = 2147483647;

pub struct Generator {
    value: u64,
    factor: u64,
    condition_factor: u64,
}

impl Generator {
    pub fn new(value: u64, factor: u64, condition_factor: u64) -> Generator {
        Generator {
            value,
            factor,
            condition_factor,
        }
    }

    pub fn generate_next(&mut self) -> u64 {
        loop {
            self.value = self.value * self.factor;
            self.value = self.value % DENOMINATOR;

            if self.value % self.condition_factor == 0 {
                break;
            }
        }

        self.value
    }
}