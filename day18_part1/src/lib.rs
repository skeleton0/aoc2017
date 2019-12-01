mod audio;

use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut f = File::open("input.txt").expect("Failed to open file.");

    let mut source = String::new();
    f.read_to_string(&mut source).expect("Failed to read file.");

    let program = audio::compile_source(&source);

    let program = match program {
        Ok(val) => val,
        Err(val) => panic!("Source compilation failed: {}", val),
    };

    let mut audio_processor = audio::Processor::new();
    audio_processor.load_program(program);

    audio_processor.run_program();
}