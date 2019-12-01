mod coordinate;

use coordinate::Coordinate;
use std::collections::HashMap;

fn main() {
    let val = 265149;

    let sum = generate_sum_spiral(val);

    println!("First value greater than {} is {}", val, sum); 
}

fn generate_sum_spiral(limit: u32) -> u32 {
    let mut coord = Coordinate::new();

    let mut spiral_map = HashMap::new();
    spiral_map.insert(coord.to_string(), 1);

    let mut positive_direction = true;
    let mut travel_limit = 1;

    loop {
        if positive_direction {
            //travel in x direction
            for _ in 0..travel_limit {
                coord.x += 1;
                let sum = sum_neighbours(&spiral_map, &coord);
                
                if sum > limit {
                    return sum 
                }

                spiral_map.insert(coord.to_string(), sum);
            }

            //travel in y direction
            for _ in 0..travel_limit {
                coord.y += 1;
                let sum = sum_neighbours(&spiral_map, &coord);

                if sum > limit {
                    return sum 
                }

                spiral_map.insert(coord.to_string(), sum);
            }
        }
        else {
            //travel in x direction
            for _ in 0..travel_limit {
                coord.x -= 1;
                let sum = sum_neighbours(&spiral_map, &coord);

                if sum > limit {
                    return sum 
                }

                spiral_map.insert(coord.to_string(), sum);
            }

            //travel in y direction
            for _ in 0..travel_limit {
                coord.y -= 1;
                let sum = sum_neighbours(&spiral_map, &coord);

                if sum > limit {
                    return sum 
                }

                spiral_map.insert(coord.to_string(), sum);
            }
        }

        travel_limit += 1;
        positive_direction = !positive_direction;
    }
}

fn sum_neighbours(map: &HashMap<String, u32>, pos: &Coordinate) -> u32 {
    let mut neighbours = Vec::new();

    //construct list of coords of the pos's neighbours
    neighbours.push(Coordinate { x: pos.x + 1, y: pos.y });
    neighbours.push(Coordinate { x: pos.x - 1, y: pos.y });
    neighbours.push(Coordinate { x: pos.x, y: pos.y + 1 });
    neighbours.push(Coordinate { x: pos.x, y: pos.y - 1 });
    neighbours.push(Coordinate { x: pos.x + 1, y: pos.y + 1 });
    neighbours.push(Coordinate { x: pos.x + 1, y: pos.y - 1 });
    neighbours.push(Coordinate { x: pos.x - 1, y: pos.y - 1 });
    neighbours.push(Coordinate { x: pos.x - 1, y: pos.y + 1 });
    
    //sum the neighbours
    let mut sum = 0;
    for neighbour in &neighbours {
        match map.get(&neighbour.to_string()) {
            Some(i) => sum += i,
            None    => (),
        }
    }

    sum
}
