mod dance_group;

use std::fs::File;
use std::io::Read;
use dance_group::Instruction;

fn read_instructions_from_file(filename: &str) -> Vec<Instruction> {
    let mut f = File::open(filename).unwrap();

    let mut file_input = String::new();
    f.read_to_string(&mut file_input).unwrap();

    let mut instructions = Vec::new();

    for instr in file_input.split(',') {
        let mut iter = instr.trim().chars();

        let instr_type = iter.next().unwrap();

        match instr_type {
            's' => {
                let mut spin_val = String::new();
                for c in iter {
                    spin_val.push(c);
                }

                let spin_val: u32 = spin_val.parse().unwrap();
                instructions.push(Instruction::Spin(spin_val));
            },
            'x' => {
                let mut exchange_val_a = String::new();
                let mut exchange_val_b = String::new();
                
                for c in iter.by_ref() {
                    if c == '/' {
                        break;
                    }

                    exchange_val_a.push(c);
                }

                for c in iter {
                    exchange_val_b.push(c);
                }

                let exchange_val_a: usize = exchange_val_a.parse().unwrap();
                let exchange_val_b: usize = exchange_val_b.parse().unwrap();

                instructions.push(Instruction::Exchange(exchange_val_a, exchange_val_b));
            },
            'p' => {
                let partner_a = iter.next().unwrap();
                iter.next(); //skip '/' character
                let partner_b = iter.next().unwrap();

                instructions.push(Instruction::Partner(partner_a, partner_b));
            },
            _ => panic!("Invalid instruction type encountered."),
            
        }
    }

    instructions
}

pub fn run() {
    let instructions = read_instructions_from_file("input.txt");

    let mut group = Vec::new();

    for i in 97..113 {
        group.push(char::from(i));
    }

    let mut group = dance_group::DanceGroup::new(group);

    group.repeat_dance(&instructions, 1_000_000_000);

    println!("Resulting group: {:?}", group.get_group());
}