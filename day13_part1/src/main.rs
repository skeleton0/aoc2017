mod firewall;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("Failed to read from file.");

    let mut fw = firewall::Firewall::new();

    //populate firewall with scanners
    for line in input.lines() {
        let mut iter = line.split(':');

        let layer = iter.next().expect("Failed to extract layer value from input.");
        let layer: usize = layer.trim().parse().expect("Failed to parse layer value to usize.");

        let range = iter.next().expect("Failed to extract range value from input.");
        let range: u32 = range.trim().parse().expect("Failed to parse range value to u32.");

        fw.add_scanner(layer, range);
    }

    let result = fw.run_simulation();

    println!("Severity of traversing firewall is {}", result);
}
