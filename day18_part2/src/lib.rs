mod audio;

use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

pub fn run() {
    let mut f = File::open("input.txt").expect("Failed to open file.");

    let mut source = String::new();
    f.read_to_string(&mut source).expect("Failed to read file.");

    let program = audio::compile_source(&source);

    let program = match program {
        Ok(val) => val,
        Err(val) => panic!("Source compilation failed: {}", val),
    };

    let mut audio_processor_0 = audio::Processor::new(0);
    let mut audio_processor_1 = audio::Processor::new(1);

    audio_processor_0.load_program(program.clone());
    audio_processor_1.load_program(program);

    let mut processor_1_tx_count = 0;
    
    let mut tx_1 = VecDeque::new();
    loop {
        audio_processor_0.send_network_tx(&mut tx_1);
        let mut tx_0 = audio_processor_0.run_program();

        audio_processor_1.send_network_tx(&mut tx_0);
        tx_1.append(&mut audio_processor_1.run_program());

        processor_1_tx_count += tx_1.len();

        if tx_1.is_empty() {
            break;
        }
    }

    println!("audio_processor_1 made {} transmissions.", processor_1_tx_count);
}