extern crate day14_part1;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");
    
    let mut file_content = String::new();

    f.read_to_string(&mut file_content).expect("Failed to read from file.");

    let mut length_sets = Vec::new();
    
    //generate vector of lengths based on input length
    for i in 0..128 {
        let row_unique_input = format!("{}-{}", file_content.trim(), i);

        let mut lengths = Vec::new();
        for ascii_code in row_unique_input.trim().bytes() {
            lengths.push(ascii_code);
        }

        length_sets.push(lengths);
    }

    //knot hash each row
    let mut knot_hashes = Vec::new();
    for lengths in length_sets.iter_mut() {
        knot_hashes.push(day14_part1::knot_hash_256_64(lengths));
    }

    let mut frag_grid = [[false; 128]; 128];

    //enter each knot hash into the frag grid
    for i in 0..knot_hashes.len() {
        let mut knot_hash_iter = knot_hashes[i].chars();

        for j in 0..knot_hashes[i].len() {
            if let '1' = knot_hash_iter.next().expect("knot_hash_iter ended prematurely.") {
                frag_grid[i][j] = true;
            }
        }
    }

    //work out how many squares are used (true) in the frag grid
    let mut used = 0;
    for i in 0..frag_grid.len() {
        for j in 0..frag_grid.len() {
            if frag_grid[i][j] {
                used += 1;
            }
        }
    }

    println!("{} squares are used in the frag grid.", used);
}
