mod hex_position;

use hex_position::HexDirection;

pub fn execute_directions(directions: &str) {
    let mut hex_pos = hex_position::HexPosition::new();

    for dir in directions.split(',') {
        match dir.trim() {
            "n"  => hex_pos.move_position(HexDirection::North),
            "ne" => hex_pos.move_position(HexDirection::NorthEast),
            "se" => hex_pos.move_position(HexDirection::SouthEast),
            "s"  => hex_pos.move_position(HexDirection::South),
            "sw" => hex_pos.move_position(HexDirection::SouthWest),
            "nw" => hex_pos.move_position(HexDirection::NorthWest),
            _    => panic!("Direction not understood."), 
        }
    }

    println!("Distance from origin: {}", hex_pos.distance_from(&hex_position::HexPosition::new()));
    println!("Max distance from origin: {}", hex_pos.max_dist_from_origin());
}