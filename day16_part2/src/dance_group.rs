pub enum Instruction {
    Spin(u32),
    Exchange(usize, usize),
    Partner(char, char),
}

pub struct DanceGroup {
    dancers: Vec<char>
}

impl DanceGroup {
    pub fn new(dancers: Vec<char>) -> DanceGroup {

        DanceGroup {
            dancers,
        }
    }

    pub fn spin(&mut self, dancers: u32) {
        let mut new_group = Vec::new();

        let spin_index = self.dancers.len() - dancers as usize;

        //add specified ending dancers to the start
        for i in spin_index..self.dancers.len() {
            new_group.push(self.dancers[i]);
        }

        //add remaining dancers to the end
        for i in 0..spin_index {
            new_group.push(self.dancers[i]);
        }

        self.dancers = new_group;
    }

    pub fn exchange(&mut self, a: usize, b: usize) {
        let temp = self.dancers[a];

        self.dancers[a] = self.dancers[b];

        self.dancers[b] = temp;
    }

    pub fn partner(&mut self, a: char, b: char) {
        let mut a_index = 0;
        let mut b_index = 0;

        for (i, c) in self.dancers.iter().enumerate() {
            if *c == a {
                a_index = i;
            }

            if *c == b {
                b_index = i;
            }
        }

        self.exchange(a_index, b_index);
    }

    pub fn get_group(&self) -> &[char] {
        &self.dancers
    }

    pub fn execute_dance(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            match *instr {
                Instruction::Spin(val) => self.spin(val),
                Instruction::Exchange(a, b) => self.exchange(a, b),
                Instruction::Partner(a, b) => self.partner(a, b),
            }
        }
    }

    pub fn repeat_dance(&mut self, instructions: &[Instruction], iterations: u32) {
        let initial_state = self.dancers.clone();

        //find how many iterations of the dance until we get a repeated initial state
        let mut repeated_state_iterations = 0;
        for i in 0..iterations {
            self.execute_dance(instructions);

            if self.dancers == initial_state {
                repeated_state_iterations = i + 1;
                break;
            }
        }

        println!("self.dancers = {:?}", self.dancers);

        println!("repeated_state_iterations = {}", repeated_state_iterations);

        //instructions don't change state so can quit here
        if repeated_state_iterations == 0 {
            return;
        }

        //amount of iterations we need to do to find the desired iteration state
        let required_iterations = iterations % repeated_state_iterations;

        println!("required_iterations = {}", required_iterations);

        //execute that amount of iterations to get to desired state
        for _ in 0..required_iterations {
            self.execute_dance(instructions);
        }
    }   
}