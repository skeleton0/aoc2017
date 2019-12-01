use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("input.txt").expect("Could not open input.txt");

    let mut input = String::new();

    f.read_to_string(&mut input).expect("Could not read file.");

    let mut valid_count = 0;

    for passphrase in input.lines() {
        let mut words = HashSet::new();
       
        let mut valid = true;
        for word in passphrase.split_whitespace() {
            if !words.insert(word) {
                valid = false;
                break;
            }
        }

        if valid {
            valid_count += 1;
        }
    }

    println!("Valid passphrases: {}", valid_count);
}
