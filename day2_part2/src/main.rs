use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open input.txt");

    let mut input = String::new();

    f.read_to_string(&mut input).expect("Failed to read file.");

    let mut sum = 0;

    for line in input.lines() {
        for numerator in line.split_whitespace() {
            let numerator: i32 = numerator.trim().parse().expect("Failed to parse string to int");

            for denominator in line.split_whitespace() {
                let denominator: i32 = denominator.trim().parse().expect("Failed to parse string to int");

                if numerator % denominator == 0 && numerator != denominator {
                    sum += numerator / denominator;
                }
            }
        }
    }

    println!("Checksum: {}", sum);
}
