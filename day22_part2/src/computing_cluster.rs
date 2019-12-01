use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

pub struct ComputingCluster {
    nodes: HashMap<(i32, i32), NodeState>,
}

impl ComputingCluster {
    pub fn new() -> ComputingCluster {
        ComputingCluster {
            nodes: HashMap::new(),
        }
    }

    pub fn get_node_state(&mut self, node: (i32, i32)) -> NodeState {
        let node_state = self.nodes.entry(node).or_insert(NodeState::Clean);

        *node_state
    }

    pub fn set_node_state(&mut self, node: (i32, i32), state: NodeState) {
        let node_state = self.nodes.entry(node).or_insert(NodeState::Clean);

        *node_state = state;
    }
}