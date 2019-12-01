use std::fs::File;
use std::io::Read;

mod instruction;
mod processor;

fn main() {
    let mut input_file = File::open("input.txt").expect("Failed to open file.");

    let mut input_data = String::new();

    input_file.read_to_string(&mut input_data).expect("Failed to read file.");

    let mut processor = processor::Processor::new();

    for msg in input_data.lines() {
        let instr = instruction::Instruction::from_msg(msg);

        processor.execute_instruction(&instr);
    }

    println!("Largest value in a register is {}.", processor.largest_register_value().expect("Processor has no registers."));
}
