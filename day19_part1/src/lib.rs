mod routing_diagram;

pub fn run() {
    let diagram = routing_diagram::read_diagram_from_file("input.txt");

    let path = routing_diagram::traverse_diagram(&diagram);

    println!("To navigate the routing diagram, the packet must follow the path: {}", path);
}