mod component_bridge;

use std::fs::File;
use std::io::Read;

fn load_bridge_components_from_file(filename: &str) -> Vec<(u32, u32)> {
    let mut f = File::open(filename).unwrap();

    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut components = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let mut it = line.split('/');

        let port1: u32 = it.next().unwrap().parse().unwrap();
        let port2: u32 = it.next().unwrap().parse().unwrap();

        components.push((port1, port2));
    }

    components
}

pub fn run() {
    let bridges = component_bridge::ComponentTree::new(load_bridge_components_from_file("input.txt"));

    println!("Strongest bridge found in the bridge tree had a strength of {}.", bridges.find_strongest_bridge(0));
}