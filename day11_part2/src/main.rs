extern crate day11_part2;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");

    let mut directions = String::new();
    f.read_to_string(&mut directions).expect("Failed to read from file.");

    day11_part2::execute_directions(&directions);
}
