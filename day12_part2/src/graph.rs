use std::collections::HashMap;

pub struct Graph {
    node_pool: HashMap<u32, Vec<u32>>, //id, connections
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            node_pool: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: u32) -> bool {
        if self.node_pool.contains_key(&id) {
            return false;
        }
        else {
            self.node_pool.insert(id, Vec::new());
            return true;
        }
    }

    pub fn add_connection(&mut self, id: u32, connection: u32) -> bool {
        if !self.node_pool.contains_key(&connection) {
            return false;
        }

        match self.node_pool.get_mut(&id) {
            Some(connections) => connections.push(connection),
            None => return false,
        }
        
        true
    }

    pub fn count_group_members(&self, root_id: u32) -> u32 {
        let mut discovered = Vec::new();

        self.unique_count_graph_traversal(root_id, &mut discovered);

        return discovered.len() as u32;
    }

    fn unique_count_graph_traversal(&self, node_id: u32, discovered: &mut Vec<u32>) {
        //if we've already discovered this node
        if discovered.contains(&node_id) {
            return;
        }
        else {
            discovered.push(node_id);
        }
        
        for connection in self.node_pool.get(&node_id).expect("Node not found in node_pool.").iter() {
            self.unique_count_graph_traversal(*connection, discovered);
        }
    }

    pub fn count_unique_groups(&self) -> u32 {
        let mut discovered = Vec::new();
        
        let mut unique_groups = 0;
        let mut discovered_len = discovered.len();

        for (node, _) in self.node_pool.iter() {
            self.unique_count_graph_traversal(*node, &mut discovered);

            if discovered.len() > discovered_len {
                unique_groups += 1;
                discovered_len = discovered.len();
            }
        }

        unique_groups
    }
}