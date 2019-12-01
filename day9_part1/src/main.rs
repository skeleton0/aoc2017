extern crate day9_part1;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");
    
    let mut f_content = String::new();
    f.read_to_string(&mut f_content).expect("Failed to read file.");

    let score = day9_part1::score_stream(&f_content);

    println!("Stream has a score of: {}", score);
}
