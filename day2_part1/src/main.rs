use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("puzzle_input.txt").expect("Failed to open puzzle_input.txt");

    let mut input = String::new();

    f.read_to_string(&mut input).expect("Failed to read from file.");

    let mut sum = 0;

    for line in input.lines() {
        let mut min = 0;
        let mut max = 0;

        //find the min and max of the row
        for num in line.split_whitespace() {
            let num: i32 = num.trim().parse().expect("Failed to parse value in file to i32");

            if num < min || min == 0 {
                min = num;
            }
            if num > max || max == 0 {
                max = num
            }
        }

        sum += max - min;
    }

    println!("checksum: {}", sum);
}

