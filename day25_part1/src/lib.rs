mod turing_machine;

pub fn run() {
    let mut machine = turing_machine::TuringMachine::new();

    const STEPS: u32 = 12459852;
    for _ in 0..STEPS {
        machine.tick();
    }

    println!("Turing machine has a checksum value of {}.", machine.get_diagnostic_checksum());
}