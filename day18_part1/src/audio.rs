use std::collections::HashMap;

pub enum Argument {
    Register(String),
    Literal(i64),
}

impl Clone for Argument {
    fn clone(&self) -> Argument {
        match self {
            &Argument::Register(ref val) => Argument::Register(val.clone() ),
            &Argument::Literal(val) => Argument::Literal(val),
        }
    }
}

pub enum Instruction {
    Snd(Argument),
    Set(Argument, Argument),
    Add(Argument, Argument),
    Mul(Argument, Argument),
    Mod(Argument, Argument),
    Rcv(Argument),
    Jgz(Argument, Argument),
}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        match self {
            &Instruction::Snd(ref a)        => Instruction::Snd(a.clone()),
            &Instruction::Set(ref a, ref b) => Instruction::Set(a.clone(), b.clone()),
            &Instruction::Add(ref a, ref b) => Instruction::Add(a.clone(), b.clone()),
            &Instruction::Mul(ref a, ref b) => Instruction::Mul(a.clone(), b.clone()),
            &Instruction::Mod(ref a, ref b) => Instruction::Mod(a.clone(), b.clone()),
            &Instruction::Rcv(ref a)        => Instruction::Rcv(a.clone()),
            &Instruction::Jgz(ref a, ref b) => Instruction::Jgz(a.clone(), b.clone()),
        }
    }
}

fn resolve_arg(arg: &str) -> Argument {
    match arg.parse::<i64>() {
        Ok(val) => Argument::Literal(val),
        Err(_) => Argument::Register(String::from(arg)),
    }
}

pub fn compile_source(source: &str) -> Result<Vec<Instruction>, String> {
    let mut program = Vec::new();

    for line in source.trim().lines() {
        let mut word = line.trim().split_whitespace();

        let instruction = match word.next() {
            Some(val) => val,
            None => continue,
        };

        let a = match word.next() {
            Some(val) => resolve_arg(val),
            None => return Err(String::from("First argument missing.")),
        };

        let b = word.next();

        //check for extra args
        match word.next() {
            Some(_) => return Err(String::from("Too many arguments.")),
            None => (),
        }

        let instruction = match instruction {
            "snd" => {
                if let Some(_) = b {
                    return Err(String::from("Too many arguments for snd instruction."));
                }

                Instruction::Snd(a)
            },
            "set" => {
                let b = match b {
                    Some(val) => resolve_arg(val),
                    None => return Err(String::from("Not enough arguments for set instruction.")),
                };

                match a {
                    Argument::Register(_) => Instruction::Set(a, b),
                    Argument::Literal(_) => return Err(String::from("Invalid first argument for set instruction.")),
                }
            },
            "add" => {
                let b = match b {
                    Some(val) => resolve_arg(val),
                    None => return Err(String::from("Not enough arguments for add instruction.")),
                };

                match a {
                    Argument::Register(_) => Instruction::Add(a, b),
                    Argument::Literal(_) => return Err(String::from("Invalid first argument for add instruction.")),
                }
            },
            "mul" => {
                let b = match b {
                    Some(val) => resolve_arg(val),
                    None => return Err(String::from("Not enough arguments for mul instruction.")),
                };

                match a {
                    Argument::Register(_) => Instruction::Mul(a, b),
                    Argument::Literal(_) => return Err(String::from("Invalid first argument for mul instruction.")),
                }
            },
            "mod" => {
                let b = match b {
                    Some(val) => resolve_arg(val),
                    None => return Err(String::from("Not enough arguments for mod instruction.")),
                };

                match a {
                    Argument::Register(_) => Instruction::Mod(a, b),
                    Argument::Literal(_) => return Err(String::from("Invalid first argument for mod instruction.")),
                }
            },
            "rcv" => {
                if let Some(_) = b {
                    return Err(String::from("Too many arguments for rcv instruction."));
                }

                Instruction::Rcv(a)
            },
            "jgz" => {
                let b = match b {
                    Some(val) => resolve_arg(val),
                    None => return Err(String::from("Not enough arguments for mod instruction.")),
                };

                Instruction::Jgz(a, b)
            },
            _ => {
                return Err(String::from("Unrecognised instruction."));
            } 
        };

        program.push(instruction);
    }

    Ok(program)
}

pub struct Processor {
    registers: HashMap<String, i64>,
    program: Vec<Instruction>,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: HashMap::new(),
            program: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    fn get_register_value(&mut self, register: String) -> i64 {
        let reg = self.registers.entry(register).or_insert(0);
        *reg
    }

    fn set_register_value(&mut self, register: String, val: i64) {
        let reg = self.registers.entry(register).or_insert(0);
        *reg = val;
    }

    fn execute_snd(&mut self, a: Argument) -> i64 {
        match a {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        }
    }

    fn execute_set(&mut self, a: Argument, b: Argument) {
        let b = match b {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        match a {
            Argument::Register(val) => self.set_register_value(val, b),
            Argument::Literal(_) => panic!("Executing set instruction on a literal."),
        }
    }

    fn execute_add(&mut self, a: Argument, b: Argument) {
        let b = match b {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        match a {
            Argument::Register(val) => {
                let a = self.get_register_value(val.clone());
                self.set_register_value(val, a + b);
            },
            Argument::Literal(_) => panic!("Executing add instruction on a literal."),
        }
    }

    fn execute_mul(&mut self, a: Argument, b: Argument) {
        let b = match b {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        match a {
            Argument::Register(val) => {
                let a = self.get_register_value(val.clone());
                self.set_register_value(val, a * b);
            },
            Argument::Literal(_) => panic!("Executing mul instruction on a literal."),
        }
    }

    fn execute_mod(&mut self, a: Argument, b: Argument) {
        let b = match b {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        match a {
            Argument::Register(val) => {
                let a = self.get_register_value(val.clone());
                self.set_register_value(val, a % b);
            },
            Argument::Literal(_) => panic!("Executing mod instruction on a literal."),
        }
    }

    fn execute_rcv(&mut self, a: Argument) -> bool {
        let a = match a {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        if a == 0 {
            false
        } else {
            true
        }
    }

    fn execute_jgz(&mut self, a: Argument, b: Argument) -> i64 {
        let b = match b {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        let a = match a {
            Argument::Register(val) => self.get_register_value(val),
            Argument::Literal(val) => val,
        };

        if a == 0 {
            1
        } else {
            b
        }
    }

    pub fn run_program(&mut self) {
        if self.program.is_empty() {
            return;
        }
        
        self.registers.clear();
        let mut instruction_pointer = 0;
        let mut last_snd_cache = 0;

        loop {
            if instruction_pointer < 0 || instruction_pointer >= self.program.len() as i64 {
                break;
            }

            let instr = self.program[instruction_pointer as usize].clone();

            match instr {
                Instruction::Snd(a)    => last_snd_cache = self.execute_snd(a),
                Instruction::Set(a, b) => self.execute_set(a, b),
                Instruction::Add(a, b) => self.execute_add(a, b),
                Instruction::Mul(a, b) => self.execute_mul(a, b),
                Instruction::Mod(a, b) => self.execute_mod(a, b),
                Instruction::Rcv(a)    => {
                    if self.execute_rcv(a) {
                        println!("Recovered last_snd_cache: {}", last_snd_cache);
                        break;
                    }
                },
                Instruction::Jgz(a, b) => {
                    instruction_pointer += self.execute_jgz(a, b);
                    //decrement instruction_pointer in anticipation of the loop's increment
                    instruction_pointer -= 1;
                },
            }

            instruction_pointer += 1;
        }
    }
}