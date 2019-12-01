use std::fs::File;
use std::io::Read;

mod tree;

fn main() {
    let mut f = File::open("input.txt").expect("Failed to open file.");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("Failed to read file.");

    let mut tree = tree::Tree::new();

    //parse lines into nodes
    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let name = iter.next().expect("Could not find name.");

        //parse weight value - is specified in the form (x) where x is an integer
        let weight = iter.next().expect("Could not find weight portion.");
        let weight = weight.trim_left_matches('(').trim_right_matches(')');
        let weight: u32 = weight.parse().expect("Failed to parse weight to u32.");

        tree.add_node(name, weight);
    }

    //connect children
    for line in input.lines() {
        let mut iter = line.split_whitespace();

        let name = iter.next().expect("Could not find name.");

        //skip weight (we don't need it for child connection) 
        iter.next(); 

        if let Some(_) = iter.next() {
            let mut children = Vec::new();

            for child in iter {
                let child = child.trim_right_matches(',');
                children.push(String::from(child));
            }

            tree.connect_children(name, &children);
        }
    }

    tree.connect_root();

    println!("{}", tree.get_root_name());
}
