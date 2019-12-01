extern crate day10_part1;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");
    
    let mut file_content = String::new();

    f.read_to_string(&mut file_content).expect("Failed to read from file.");

    let mut lengths = Vec::new();
    for val in file_content.trim().split(',') {
        let val: u32 = val.parse().expect("Failed to parse length to u32.");
        lengths.push(val);
    }

    let result = day10_part1::knot_hash_256(&lengths);

    println!("Result of hash: {}", result);
}
