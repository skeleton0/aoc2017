mod coprocessor;

use coprocessor::Instruction;
use coprocessor::Coprocessor;

use std::fs::File;
use std::io::Read;

fn load_program_from_file(filename: &str) -> Vec<Instruction> {
    let mut file = File::open(filename).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut program = Vec::new();

    for line in content.lines() {
        let mut words = line.split_whitespace();

        let instruction = words.next().unwrap();
        let register = words.next().unwrap().chars().next().unwrap();
        let value = words.next().unwrap();

        let instruction = match instruction {
            "set" => Instruction::Set(register, String::from(value)),
            "sub" => Instruction::Sub(register, String::from(value)),
            "mul" => Instruction::Mul(register, String::from(value)),
            "jnz" => Instruction::Jnz(register.to_string(), String::from(value)),
            _ => panic!(),
        };

        program.push(instruction);
    }

    program
}

pub fn run() {
    let program = load_program_from_file("input.txt");

    let mut processor = Coprocessor::new();

    processor.run_program(&program);

    println!("Processor made {} mul invocations.", processor.query_mul_invocations());
}
