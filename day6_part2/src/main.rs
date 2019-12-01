use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut fs = File::open("input.txt").expect("Failed to open input.txt");

    let mut input = String::new();
    fs.read_to_string(&mut input).expect("Failed to read file.");

    let mut memory_banks = Vec::new();

    //populate memory_banks with values from file
    for bank in input.split_whitespace() {
        let bank: u32 = bank.parse().expect("Failed to parse bank to u32.");

        memory_banks.push(bank);
    }

    let mut configs = HashSet::new();
    let mut config_history = vec![banks_to_string(&memory_banks)];

    while configs.insert(banks_to_string(&memory_banks)) {
        let index = find_max(&memory_banks);

        let mut blocks = memory_banks[index];

        memory_banks[index] = 0;

        let mut alloc_index = index + 1;
        while blocks > 0 {
            match memory_banks.get_mut(alloc_index) {
                Some(bank) => {
                    *bank += 1;
                    blocks -= 1;
                    alloc_index += 1;
                },
                
                None       => alloc_index = 0,
            }
        }

        config_history.push(banks_to_string(&memory_banks));
    }

    //find amount of block redistributions between duplicates
    let mut index = 0;
    for i in 0..config_history.len() {
       if config_history[i] == config_history[config_history.len() - 1] {
           index = i;
           break;
       }
    }

    let diff = config_history.len() - 1 - index;

    println!("{} block redistributions in infinite loop.", diff);
}

fn find_max(banks: &Vec<u32>) -> usize {
    let mut max = 0;

    for i in 1..banks.len() {
        if banks[i] > banks[max] {
            max = i;
        }
    }

    max
}

fn banks_to_string(banks: &Vec<u32>) -> String {
    let mut bank_str = String::new();

    for bank in banks {
        bank_str.push_str(&bank.to_string());
        bank_str.push(' ');
    }

    bank_str
}
