#[derive(Debug)]
struct Node {
    parent_id: Option<usize>,
    children_ids: Vec<usize>,
    name: String,
    weight: u32,
}

impl Node {
    fn new(name: &str, weight: u32) -> Node {
        Node {
            parent_id: None,
            children_ids: vec![],
            name: String::from(name),
            weight,
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    node_pool: Vec<Node>,
    root_id: Option<usize>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            node_pool: vec![],
            root_id: None,
        }
    }
    pub fn add_node(&mut self, name: &str, weight: u32) {
        //ensure this node's name doesn't already exist in the pool
        for node in &self.node_pool {
            if node.name == name {
                panic!("Tried to add node but node with same name already exists.");
            }
        }

        self.node_pool.push(Node::new(name, weight));
    }

    pub fn connect_children(&mut self, name: &str, children_names: &Vec<String>) {
        if children_names.contains(&String::from(name)) {
            panic!("Parent cannot be a child to itself.");
        }

        let mut children_not_connected = children_names.clone();

        for i in 0..self.node_pool.len() {
            //if this node is the parent node we're looking for
            if self.node_pool[i].name == name {
                for j in 0..self.node_pool.len() {
                    //if node is one of the children we're looking for
                    if children_names.contains(&self.node_pool[j].name) {
                        //remove child from not connected vector
                        for k in 0..children_not_connected.len() {
                            if children_not_connected[k] == self.node_pool[j].name {
                                children_not_connected.remove(k);
                                break;
                            }
                        }

                        //connect child id to parent
                        self.node_pool[i].children_ids.push(j);

                        //connect parent id to child
                        match self.node_pool[j].parent_id {
                            Some(_) => panic!("Tried to add child to parent, but child already has a parent."),
                            None => self.node_pool[j].parent_id = Some(i),
                        }
                    }
                }
                
                if !children_not_connected.is_empty() {
                    panic!("Some children were not found.");
                    }

               return; 
            }
        }

        panic!("Could not find the parent node.")
    }

    pub fn connect_root(&mut self) {
        if let Some(_) = self.root_id {
            panic!("Tree already has root.");
        }

        for i in 0..self.node_pool.len() {
            if let None = self.node_pool[i].parent_id {
                match self.root_id {
                    Some(_) => panic!("Found multiple possible roots."),
                    None => self.root_id = Some(i),
                }
            }
        }
    }

    pub fn get_root_name(&self) -> &str {
        match self.root_id {
            Some(id) => return &self.node_pool[id].name,
            None => panic!("Tree has no root."),
        }
    }

    fn sum_children(&mut self, node_id: usize) {
        let children = &self.node_pool[node_id].children_ids.clone();

        for child in children {
            self.sum_children(*child);
            self.node_pool[node_id].weight += self.node_pool[*child].weight;
        }
    }

    pub fn sum_tree(&mut self) {
        let root_id = self.root_id.expect("Tree has no root id set.");
        self.sum_children(root_id);
    }

    fn find_imbalance(&self, node_id: usize) -> bool {
        let children = self.node_pool[node_id].children_ids.clone();
        let mut weights = Vec::new();
        let mut required_weight = 0;

        //find required weight
        for child in &children {
            let child_weight = self.node_pool[*child].weight;
            
            if weights.contains(&child_weight) {
                required_weight = child_weight;
                break;
            } 
            else {
                weights.push(child_weight);    
            }
        }

        //find which child had that weight
        for child in &children {
            if self.node_pool[*child].weight != required_weight {
                if !self.find_imbalance(*child) {
                    println!("Imbalanced child: {}\nRequired weight: {}\n", self.node_pool[*child].name, required_weight);
                }

                return true;
            }
        }

        false
    }

    pub fn print_imbalanced_node(&self) {
        let root_id = self.root_id.expect("Tree has no root id set.");
        self.find_imbalance(root_id);
    }

    fn depth_first_print(&self, node: usize, indent: u32) {
        let mut indent_string = String::new();
        for _ in 0..indent {
            indent_string.push_str("    ");
        }

        println!("{}{}({})", indent_string, self.node_pool[node].name, self.node_pool[node].weight);

        let children = self.node_pool[node].children_ids.clone();

        for child in &children {
            self.depth_first_print(*child, indent + 1);
        }

    }

    pub fn print_tree(&self) {
        let root_id = self.root_id.expect("Tree has no root id set.");
        self.depth_first_print(root_id, 0);
    }
}
