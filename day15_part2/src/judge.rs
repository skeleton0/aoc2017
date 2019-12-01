use generator;

pub struct Judge {
    generator_a: generator::Generator,
    generator_b: generator::Generator,
}

impl Judge {
    pub fn new(generator_a: generator::Generator, generator_b: generator::Generator) -> Judge {
        Judge {
            generator_a,
            generator_b,
        }
    }

    pub fn run_generator_duel(&mut self, iterations: u32) -> u32 {
        let mut matches = 0;

        for _ in 0..iterations {
            let a_output = self.generator_a.generate_next();
            let b_output = self.generator_b.generate_next();

            if a_output as u16 == b_output as u16 {
                matches += 1;
            }
        }

        matches
    }
}