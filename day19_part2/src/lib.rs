mod routing_diagram;

pub fn run() {
    let diagram = routing_diagram::read_diagram_from_file("input.txt");

    let steps = routing_diagram::traverse_diagram(&diagram);

    println!("The routing diagram took {} steps to navigate.", steps);
}