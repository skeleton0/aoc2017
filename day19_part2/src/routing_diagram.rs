use std::fs::File;
use std::io::Read;

struct Coordinate {
    x: i32,
    y: i32,
}

enum Direction {
    North,
    East,
    South,
    West,
}

pub fn print_diagram(diagram: &Vec<Vec<char>>) {
        for y in 0..diagram[0].len() {
        let mut line = String::new();
        
        for x in 0..diagram.len() {
            line.push(diagram[x][y]);
        }

        println!("{}", line);
    }
}

pub fn read_diagram_from_file(filename: &str) -> Vec<Vec<char>> {
    let mut f = File::open(filename).unwrap();

    let mut raw_diagram = String::new();
    f.read_to_string(&mut raw_diagram).unwrap();

    //find width of diagram
    let width = raw_diagram.lines().next().unwrap().chars().count() - 1; //-1 because we don't want to include \n character

    //find height of diagram
    let height = raw_diagram.lines().count();

    //init 2D vec with these dimensions
    let mut diagram = vec![vec![' '; height]; width];

    //read raw_diagram into 2D vec diagram
    for (y, line) in raw_diagram.lines().enumerate() {
        for(x, c) in line.chars().enumerate() {
            if x >= diagram.len() {
                break;
            } else {
                diagram[x][y] = c;
            }
        }
    }

    diagram
}

fn get_symbol(diagram: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if x >= diagram.len() as i32 || x < 0 || y >= diagram[0].len() as i32 || y < 0 {
        None
    } else {
        Some(diagram[x as usize][y as usize])
    }
}

pub fn traverse_diagram(diagram: &Vec<Vec<char>>) -> u32 {
    let mut pos = Coordinate { x: 0, y: 0 };

    //find entry
    for x in 0..diagram.len() {
        if let Some(c) = get_symbol(diagram, x as i32, 0) {
            if c == '|' {
                pos.x = x as i32;
            }
        }
    }

    let mut current_dir = Direction::South;
    let mut trail = String::new();
    let mut steps = 0;

    //main diagram traversal loop
    loop {
        let symbol = match get_symbol(diagram, pos.x, pos.y) {
            Some(c) if c == ' ' => break,
            Some(c) => c,
            None => break,
        };

        steps += 1;

        if symbol == '|' || symbol == '-' || symbol.is_alphabetic() {
            if symbol.is_alphabetic() {
                trail.push(symbol);
            }

            match current_dir {
                Direction::North => pos.y -= 1,
                Direction::East  => pos.x += 1,
                Direction::South => pos.y += 1,
                Direction::West  => pos.x -= 1,
            }
        } 
        else if symbol == '+' {
            let mut look_east_west = false;

            match current_dir {
                Direction::North => look_east_west = true,
                Direction::South => look_east_west = true,
                _ => ()
            }

            if look_east_west {
                let east_char = get_symbol(diagram, pos.x + 1, pos.y);
                let west_char = get_symbol(diagram, pos.x - 1, pos.y);

                if let Some(c) = east_char {
                    if c == '-' {
                        pos.x += 1;
                        current_dir = Direction::East;
                    }
                }
                if let Some(c) = west_char {
                    if c == '-' {
                        pos.x -= 1;
                        current_dir = Direction::West;
                    }
                }
            } else {
                let north_char = get_symbol(diagram, pos.x, pos.y - 1);
                let south_char = get_symbol(diagram, pos.x, pos.y + 1);

                if let Some(c) = north_char {
                    if c == '|' {
                        pos.y -= 1;
                        current_dir = Direction::North;
                    }
                }
                if let Some(c) = south_char {
                    if c == '|' {
                        pos.y += 1;
                        current_dir = Direction::South;
                    }
                }
            }
        }
    }

    steps
}