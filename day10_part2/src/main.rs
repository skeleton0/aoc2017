extern crate day10_part2;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");
    
    let mut file_content = String::new();

    f.read_to_string(&mut file_content).expect("Failed to read from file.");

    let mut lengths = Vec::new();
    for ascii_code in file_content.trim().bytes() {
        lengths.push(ascii_code);
    }

    let hash = day10_part2::knot_hash_256_64(&mut lengths);

    println!("{}", hash);
}
