mod three_dimensional;
mod particle;

use std::fs::File;
use std::io::Read;
use three_dimensional::ThreeDimensionalI32;
use particle::Particle;
use std::collections::HashMap;
use std::collections::HashSet;

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

    let mut collisions = HashMap::new();

    for (i, particle) in particles.iter().enumerate() {
        for index in (i+1)..particles.len() {
            match particle.collision_check(&particles[index]) {
                Some((time1, time2)) => {
                    {
                    let vec = collisions.entry(time1).or_insert(HashSet::new());
                    vec.insert(i);
                    vec.insert(index);
                    }

                    let vec = collisions.entry(time2).or_insert(HashSet::new());
                    vec.insert(i);
                    vec.insert(index);

                    //println!("Collision found with particle {} and {} at {}s and {}s.", i, index, time1, time2)
                },
                None => (),
            }
        }
        
    }

    let mut seen = HashSet::new();
    let mut collision_count = 0;

    for (time, particles) in collisions.iter() {
        let mut particle_list = String::from(format!("{}s: ", time).as_str());
        
        for particle in particles.iter() {
            if !seen.insert(particle) {
                println!("Have seen particle {} before.", particle);
            }

            particle_list.push_str(format!("{}, ", particle).as_str());

            collision_count += 1;
        }

        println!("{}", particle_list);
    }

    println!("Recorded {} collisions, which leaves {} particles.", collision_count, 1000-collision_count);
}