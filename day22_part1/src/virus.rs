use computing_cluster::ComputingCluster;

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Virus<'a> {
    direction: Direction,
    position: (i32, i32),
    cluster: &'a mut ComputingCluster,
    infections_caused: u32,
}

impl<'a> Virus<'a> {
    pub fn new(direction: Direction, position: (i32, i32), cluster: &mut ComputingCluster) -> Virus {
        Virus {
            direction,
            position,
            cluster,
            infections_caused: 0,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North
        }
    }

    fn change_direction(&mut self) {
        if self.cluster.get_node_state(self.position) {
            self.turn_right();
        } else {
            self.turn_left();
        }
    }

    fn change_node_state(&mut self) {
        if self.cluster.get_node_state(self.position) {
            self.cluster.set_node_state(self.position, false);
        } else {
            self.cluster.set_node_state(self.position, true);
            self.infections_caused += 1;
        }
    }

    fn move_node(&mut self) {
        match self.direction {
            Direction::North => self.position.1 -= 1,
            Direction::East => self.position.0 += 1,
            Direction::South => self.position.1 += 1,
            Direction::West => self.position.0 -= 1,
        }
    }

    pub fn run_burst(&mut self) {
        self.change_direction();
        self.change_node_state();
        self.move_node();
    }

    pub fn get_infections_caused(&self) -> u32 {
        self.infections_caused
    }
}