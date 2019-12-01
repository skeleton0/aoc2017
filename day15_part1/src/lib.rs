mod generator;
mod judge;

use std::fs::File;
use std::io::Read;

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;
const RUN_ITERATIONS: u32 = 40_000_000;

fn extract_val_from_line(line: &str) -> u64 {
    let mut words = line.split_whitespace();

    //skip 4 words
    for _ in 0..4 {
        words.next();
    }
        
    words.next().unwrap().trim().parse().unwrap()
}

fn get_starting_values(filename: &str) -> (u64, u64) {
    let mut f = File::open(filename).unwrap();

    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut iter = input.lines();

    (extract_val_from_line(iter.next().unwrap()), extract_val_from_line(iter.next().unwrap()))
}

pub fn run() {
    let (a, b) = get_starting_values("input.txt");

    let generator_a = generator::Generator::new(a, A_FACTOR);
    let generator_b = generator::Generator::new(b, B_FACTOR);

    let mut a_judge = judge::Judge::new(generator_a, generator_b);

    let result = a_judge.run_generator_duel(RUN_ITERATIONS);

    println!("Generator duel returned with {} matches.", result);
}