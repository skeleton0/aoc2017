mod rule;

use std::fs::File;
use std::io::Read;

fn get_rules_from_file(filename: &str) -> Vec<rule::Rule> {
    let mut f = File::open(filename).unwrap();

    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut rules = Vec::new();

    for line in input.lines() {
        let mut pattern_it = line.split("=>");

        let raw_pattern = pattern_it.next().unwrap().trim();
        let raw_output_pattern = pattern_it.next().unwrap().trim();

        //lambda for parsing raw pattern string to pattern vector
        let raw_to_pattern = |raw_pattern: &str| {
            let mut pattern = Vec::new();

            for c in raw_pattern.chars() {
                match c {
                    '.' => pattern.push(false),
                    '#' => pattern.push(true),
                    _ => (),
                }
            }

            pattern
        };

        let pattern = raw_to_pattern(raw_pattern);
        let output_pattern = raw_to_pattern(raw_output_pattern);

        let rule = rule::Rule::new(pattern, output_pattern);
        rules.push(rule);
    }

    rules
}

fn print_pattern(pattern: &[bool]) {
    let size = (pattern.len() as f32).sqrt();
    let size = size as usize;

    for i in 0..pattern.len() {
        if i % size == 0 {
            println!();
        }

        if pattern[i] {
            print!("#");
        } else {
            print!(".");
        }
    }

    println!();
}

fn split_pattern(pattern: &[bool]) -> Vec<Vec<bool>> {
    let size = (pattern.len() as f32).sqrt();
    let size = size as u32;

    let factor;
    if size % 2 == 0 {
        factor = 2;
    } else if size % 3 == 0 {
        factor = 3;
    } else {
        panic!("Cannot split pattern with a size that is not divisible by 2 or 3.");
    }

    let mut chunks = Vec::new();

    let mut chunk_column = 0;
    let mut chunk_row = 0;

    let amount_of_chunks = (size / factor).pow(2);
    let mut new_chunk = Vec::new();

    while (chunks.len() as u32) < amount_of_chunks {
        
        let start_pos = chunk_row * size + chunk_column * factor;
        
        for pos in start_pos..(start_pos + factor) {
            new_chunk.push(*&pattern[pos as usize]);
        }

        chunk_row += 1;
        if chunk_row % factor == 0 {
            chunks.push(new_chunk.clone());
            new_chunk.clear();

            chunk_row -= factor;
            chunk_column += 1;
            
            if chunk_column * factor == size {
                chunk_row += factor;
                chunk_column = 0;
            }
        }
    }

    chunks
}

fn join_chunks(chunks: &[Vec<bool>]) -> Vec<bool> {
    let pattern_size = (chunks.len() as f32).sqrt() * (chunks[0].len() as f32).sqrt();
    let pattern_size = pattern_size as u32;

    let mut pattern = vec![false; pattern_size.pow(2) as usize];

    let chunk_size = (chunks[0].len() as f32).sqrt();
    let chunk_size = chunk_size as u32;

    let mut pattern_row = 0;
    let mut pattern_column = 0;

    for chunk in chunks {

        for pixel_state in chunk {
            let pos = pattern_row * pattern_size + pattern_column;

            pattern[pos as usize] = *pixel_state;

            pattern_column += 1;
            if pattern_column % chunk_size == 0 {
                pattern_column -= chunk_size;
                pattern_row += 1;
            }
        }

        pattern_column += chunk_size;

        if pattern_column == pattern_size {
            pattern_column = 0;
        } else { 
            pattern_row -= chunk_size;
        }
    }

    pattern
}

pub fn run() {
    let rules = get_rules_from_file("input.txt");

    let mut pattern = vec![false, true, false, false, false, true, true, true, true];

    for _ in 0..5 {
        //split pattern
        let pattern_chunks = split_pattern(pattern.as_slice());

        //build set of chunks based on rules for each pattern_chunk
        let mut transformed_chunks = Vec::new();
        for chunk in &pattern_chunks {
            for rule in &rules {
                if rule.pattern_matches(chunk.as_slice()) {
                    transformed_chunks.push(Vec::from(rule.get_output_pattern()));
                    break;
                }
            }
        }

        //join chunks back together
        pattern = join_chunks(&transformed_chunks);
    }

    let mut enabled_pixel_count = 0;

    for pixel in &pattern {
        if *pixel {
            enabled_pixel_count += 1;
        }
    }

    println!("{} pixels are enabled in the final pattern.", enabled_pixel_count);
}