mod graph;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");

    let mut input = String::new();

    f.read_to_string(&mut input).expect("Failed to read from file.");

    let mut village = graph::Graph::new();

    //Populate the village with the nodes
    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let node = iter.next().expect("Unable to extract node id from input line.");
        let node: u32 = node.trim().parse().expect("Failed to parse node id to u32.");

        if !village.add_node(node) {
            panic!("Tried to add node but node already exists.");
        }
    }

    //Establish the connections
    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let root_node = iter.next().expect("Failed to extract node_id from input line.");
        let root_node: u32 = root_node.trim().parse().expect("Failed to parse node id to u32.");

        iter.next(); //skip '<->'

        //for each connection
        for connection in iter {
            let connection = connection.trim().trim_right_matches(',');
            let connection: u32 = connection.parse().expect("Failed to parse connection id to u32.");

            if !village.add_connection(root_node, connection) {
                panic!("Failed to make node connection as either root node or connection node does not exist.");
            }
        }
    }

    let root_node = 0;
    let members = village.count_group_members(root_node);
    println!("Count of members in group containing node {}: {}", root_node, members);
}
