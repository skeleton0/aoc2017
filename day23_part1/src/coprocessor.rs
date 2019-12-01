use std::collections::HashMap;
use std::char;

pub enum Instruction {
    Set(char, String),
    Sub(char, String),
    Mul(char, String),
    Jnz(String, String),
}

pub struct Coprocessor {
    registers: HashMap<char, i32>,
    mul_invocations: u32,
}

impl Coprocessor {
    pub fn new() -> Coprocessor {
        let mut processor = Coprocessor {
            registers: HashMap::new(),
            mul_invocations: 0,
        };

        //populate the registers
        for ascii_value in 97..105 {
            processor.registers.insert(char::from_u32(ascii_value).unwrap(), 0);
        }

        processor
    }

    fn resolve_instruction_arg(&self, arg: &str) -> i32 {
        let first_char = arg.chars().next().unwrap();

        if first_char.is_alphabetic() {
            *self.registers.get(&first_char).unwrap()
        } else {
            arg.parse().unwrap()
        }
    }
    
    fn set(&mut self, register: char, value: &str) {
        let value = self.resolve_instruction_arg(value);
        
        println!("set {} {}", register, value);

        self.registers.insert(register, value);
    }

    fn sub(&mut self, register: char, value: &str) {
        let value = self.resolve_instruction_arg(value);

        println!("sub {} {}", register, value);

        let register = self.registers.get_mut(&register).unwrap();

        *register -= value;
    }

    fn mul(&mut self, register: char, value: &str) {
        let value = self.resolve_instruction_arg(value);

        println!("mul {} {}", register, value);

        let register = self.registers.get_mut(&register).unwrap();

        *register *= value;

        self.mul_invocations += 1;
    }

    fn jnz(&self, condition: &str, value: &str) -> bool {
        let condition = self.resolve_instruction_arg(condition);

        println!("jnz {} {}", condition, value);

        condition != 0
    }

    fn clear_processor_memory(&mut self) {
        //clear registers
        for (_, value) in self.registers.iter_mut() {
            *value = 0;
        }

        //clear stats
        self.mul_invocations = 0;
    }

    pub fn run_program(&mut self, program: &Vec<Instruction>) {
        let mut instruction_ptr = 0;
        self.clear_processor_memory();

        while instruction_ptr > -1 && (instruction_ptr as usize) < program.len() {
            match program[instruction_ptr as usize] {
                Instruction::Set(ref register, ref value) => self.set(register.clone(), value.as_str()),
                Instruction::Sub(ref register, ref value) => self.sub(register.clone(), value.as_str()),
                Instruction::Mul(ref register, ref value) => self.mul(register.clone(), value.as_str()),
                Instruction::Jnz(ref condition, ref value) => {
                    if self.jnz(condition.as_str(), value.as_str()) {
                        instruction_ptr += self.resolve_instruction_arg(value.as_str()) - 1;
                    }
                },
            }

            instruction_ptr += 1;
        }
    }

    pub fn query_mul_invocations(&self) -> u32 {
        self.mul_invocations
    }
}
