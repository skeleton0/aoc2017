use std::collections::HashMap;

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

enum Direction {
    Right,
    Left,
}

pub struct TuringMachine {
    tape: HashMap<i32, bool>,
    head: i32,
    state: State,
}

impl TuringMachine {
    pub fn new() -> TuringMachine {
        TuringMachine {
            tape: HashMap::new(),
            head: 0,
            state: State::A,
        }
    }

    fn transition_state(&mut self, write: bool, move_direction: Direction, next_state: State) {
        self.tape.insert(self.head, write);
        
        if let Direction::Right = move_direction {
            self.head += 1;
        } else {
            self.head -= 1;
        }

        self.state = next_state;
    }

    pub fn tick(&mut self) {
        let tape_value = self.tape.entry(self.head).or_insert(false).clone();

        match self.state {
            State::A => {
                if tape_value {
                    self.transition_state(true, Direction::Left, State::E);
                } else {
                    self.transition_state(true, Direction::Right, State::B);
                }
            },
            State::B => {
                if tape_value {
                    self.transition_state(true, Direction::Right, State::F);
                } else {
                    self.transition_state(true, Direction::Right, State::C);
                }
            },
            State::C => {
                if tape_value {
                    self.transition_state(false, Direction::Right, State::B);
                } else {
                    self.transition_state(true, Direction::Left, State::D);
                }
            },
            State::D => {
                if tape_value {
                    self.transition_state(false, Direction::Left, State::C);
                } else {
                    self.transition_state(true, Direction::Right, State::E);
                }
            },
            State::E => {
                if tape_value {
                    self.transition_state(false, Direction::Right, State::D);
                } else {
                    self.transition_state(true, Direction::Left, State::A);
                }
            },
            State::F => {
                if tape_value {
                    self.transition_state(true, Direction::Right, State::C);
                } else {
                    self.transition_state(true, Direction::Right, State::A);
                }
            },
        }
    }

    pub fn get_diagnostic_checksum(&self) -> u32 {
        let mut checksum = 0;

        for (_, tape_value) in self.tape.iter() {
            if *tape_value {
                checksum += 1;
            }
        }

        checksum
    }
}