use std::fs::File;
use std::io::Read;

mod computing_cluster;
mod virus;

use computing_cluster::ComputingCluster;

fn read_cluster_from_file(filename: &str) -> ComputingCluster {
    let mut f = File::open(filename).unwrap();

    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut new_cluster = ComputingCluster::new();

    for (y, line) in input.lines().enumerate() {
        for (x, state) in line.chars().enumerate() {
            if let '#' = state {
                new_cluster.set_node_state((x as i32, y as i32), true);
            }
        }
    }

    new_cluster
}

pub fn run() {
    let mut cluster = read_cluster_from_file("input.txt");

    let mut virus = virus::Virus::new(virus::Direction::North, (12, 12), &mut cluster);

    for _ in 0..10000 {
        virus.run_burst();
    }

    println!("After 10k bursts, {} nodes became infected.", virus.get_infections_caused());
}