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
    let mut bridge_components = load_bridge_components_from_file("input.txt");
    let bridges = component_bridge::ComponentTree::new(bridge_components.clone());

    let longest_bridges = bridges.find_longest_bridge(0, &Vec::new());
    
    let mut strongest_bridge_strength = 0;

    bridge_components.push((0, 0));
    for longest_bridge in longest_bridges.iter() {
        let mut bridge_strength = 0;

        for bridge_component in longest_bridge {
            bridge_strength += bridge_components[*bridge_component].0 + bridge_components[*bridge_component].1;
            print!("({}, {})--", bridge_components[*bridge_component].0, bridge_components[*bridge_component].1)
        }

        println!("\n");

        if bridge_strength > strongest_bridge_strength {
            strongest_bridge_strength = bridge_strength;
        }
    }

    println!("The strongest longest bridge has a strength of {}.", strongest_bridge_strength);
}