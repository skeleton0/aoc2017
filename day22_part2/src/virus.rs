use computing_cluster::ComputingCluster;
use computing_cluster::NodeState;

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

    fn reverse_direction(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn change_direction(&mut self) {
        match self.cluster.get_node_state(self.position) {
            NodeState::Clean => self.turn_left(),
            NodeState::Weakened => (),
            NodeState::Infected => self.turn_right(),
            NodeState::Flagged => self.reverse_direction(),
        }
    }

    fn change_node_state(&mut self) {
        match self.cluster.get_node_state(self.position) {
            NodeState::Clean => self.cluster.set_node_state(self.position, NodeState::Weakened),
            NodeState::Weakened => {
                self.cluster.set_node_state(self.position, NodeState::Infected);
                self.infections_caused += 1;
            },
            NodeState::Infected => self.cluster.set_node_state(self.position, NodeState::Flagged),
            NodeState::Flagged => self.cluster.set_node_state(self.position, NodeState::Clean),
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