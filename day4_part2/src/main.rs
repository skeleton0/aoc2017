use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("input.txt").expect("Could not open input.txt");

    let mut input = String::new();

    f.read_to_string(&mut input).expect("Could not read file.");

    let mut valid = 0;

    //for each passphrase in the file
    for pass in input.lines() {
        let mut words = Vec::new();

        //populate vector with each word of the passphrase
        for word in pass.split_whitespace() {
            words.push(String::from(word));
        }

        //check if any anagrams exist in the passphrase's words
        if !contains_anagram(&words) {
            valid += 1;
        }
    }

    println!("{} valid passphrases found.", valid);
}

fn contains_anagram(words: &Vec<String>) -> bool {
    for word in words {

        let mut char_count = HashMap::new();

        //populate char_count with chars of word
        for c in word.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }

        let mut found_duplicate = false;
        for other_word in words {
            if !found_duplicate && other_word == word {
                found_duplicate = true;
                continue;
            }

            let mut char_count = char_count.clone();

            for c in other_word.chars() {
                let count = char_count.entry(c).or_insert(0);
                *count -= 1;
            }

            let mut found_anagram = true;

            for (_, count) in &char_count {
                if *count != 0 {
                    found_anagram = false;
                    break;
                }
            }

            if found_anagram {
                return true;
            }
        }
    }

    false
}
