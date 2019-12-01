mod three_dimensional;
mod particle;

use std::fs::File;
use std::io::Read;
use three_dimensional::ThreeDimensionalI32;
use particle::Particle;
use std::cmp::Ordering;

fn extract_quantity_from_string_format(content: &str, dimension_specifier: &str) -> ThreeDimensionalI32 {
    let mut dimension_specifier = String::from(dimension_specifier);
    dimension_specifier.push_str("=<"); 

    let mut content = content.trim_left_matches(&dimension_specifier);
    content = content.trim_right_matches('>');
    let mut content = content.split(',');
    
    ThreeDimensionalI32::new(content.next().unwrap().parse().unwrap(), 
                             content.next().unwrap().parse().unwrap(),
                             content.next().unwrap().parse().unwrap())
}

fn read_particles_from_file(filename: &str) -> Vec<Particle> {
    let mut f = File::open(filename).unwrap();

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents).unwrap();

    let mut particles = Vec::new();
    for line in file_contents.lines() {
        let mut quantities = line.trim().split(", ");

        let position = extract_quantity_from_string_format(quantities.next().unwrap(), "p");
        let velocity = extract_quantity_from_string_format(quantities.next().unwrap(), "v");
        let acceleration = extract_quantity_from_string_format(quantities.next().unwrap(), "a");

        particles.push(Particle::new(acceleration, velocity, position));
    }

    particles
}

pub fn run() {
    let particles = read_particles_from_file("input.txt");

    let mut closest = 0;
    for (i, particle) in particles.iter().enumerate() {
        if let Ordering::Less = particle.long_term_proximity_to_origin(&particles[closest]) {
            closest = i;
        }
    }

    println!("Closest particle found: {}", closest);
}