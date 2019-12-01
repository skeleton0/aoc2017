mod coordinate;

use coordinate::Coordinate;

fn main() {
    let val = 265149;

    let coord = generate_spiral_to_val(val);

    println!("Coordinate for {} is ({}, {})", val, coord.x, coord.y);
    println!("Which gives a Manhattan Distance of {}", coord.x.abs() + coord.y.abs());
}

fn generate_spiral_to_val(val: u32) -> Coordinate {

    let mut coord = Coordinate::new();
    let mut cell_value = 1;
    let mut positive_direction = true;
    let mut travel_limit = 1;

    loop {
        if positive_direction {
            //traverse in x dimension
            for _ in 0..travel_limit {
                coord.x += 1;
                cell_value += 1;

                if cell_value == val {
                    return coord
                }
            }

            //traverse in y dimension
            for _ in 0..travel_limit {
                coord.y += 1;
                cell_value += 1;

                if cell_value == val {
                    return coord
                }
            }
        }
        else {
            //traverse in x dimension
            for _ in 0..travel_limit {
                coord.x -= 1;
                cell_value += 1;

                if cell_value == val {
                    return coord
                }
            }

            //traverse in y dimension
            for _ in 0..travel_limit {
                coord.y -= 1;
                cell_value += 1;

                if cell_value == val {
                    return coord
                }
            }
        }

        travel_limit += 1;
        positive_direction = !positive_direction;
    }
}
