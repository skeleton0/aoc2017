use std::collections::HashMap;

#[derive(Debug)]
pub struct ComputingCluster {
    nodes: HashMap<(i32, i32), bool>,
}

impl ComputingCluster {
    pub fn new() -> ComputingCluster {
        ComputingCluster {
            nodes: HashMap::new(),
        }
    }

    pub fn get_node_state(&mut self, node: (i32, i32)) -> bool {
        let node_state = self.nodes.entry(node).or_insert(false);

        *node_state
    }

    pub fn set_node_state(&mut self, node: (i32, i32), state: bool) {
        let node_state = self.nodes.entry(node).or_insert(false);

        *node_state = state;
    }
}