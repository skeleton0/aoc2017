use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Failed to read file.");

    //construct maze
    let mut maze = Vec::new();
    for offset in input.lines() {
        let offset: i32 = offset.trim().parse().expect("Failed to parse offset from file to i32.");

        maze.push(offset);
    }

    let mut steps = 0;
    let mut ip: i32 = 0; //instruction pointer

    loop {
        if ip < 0 {
            break;
        }

        let offset = match maze.get_mut(ip as usize) {
            Some(val) => val,
            None => break,
        };

        ip += *offset;

        *offset += 1;

        steps += 1;
    }

    println!("Maze took {} steps to escape.", steps);
}
