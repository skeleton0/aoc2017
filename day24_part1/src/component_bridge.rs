use std::collections::VecDeque;

struct ComponentNode {
    component_index: usize,
    available_port: u32,
    available_component_indices: Vec<usize>,
    children_indices: Vec<usize>,
}

impl ComponentNode {
    fn new(component_index: usize, available_port: u32, parent_component_indices: &Vec<usize>) -> ComponentNode {
        let mut available_component_indices = parent_component_indices.clone();

        //remove the component used by this node from the pool
        for (i, component) in parent_component_indices.iter().enumerate() {
            if *component == component_index {
                available_component_indices.remove(i);
                break;
            }
        }

        ComponentNode {
            component_index,
            available_port,
            available_component_indices,
            children_indices: Vec::new(),
        }
    }

    fn add_child(&mut self, child_index: usize) {
        self.children_indices.push(child_index);
    }
}

pub struct ComponentTree {
    components: Vec<(u32, u32)>,
    nodes: Vec<ComponentNode>,
}

impl ComponentTree {
    pub fn new(components: Vec<(u32, u32)>) -> ComponentTree {
        let mut tree = ComponentTree {
            components,
            nodes: Vec::new(),
        };

        //push root component into tree
        tree.components.push((0, 0)); 

        //construct an available component indices vector
        let mut available_component_indices = Vec::new();
        for (i, _) in tree.components.iter().enumerate() {
            available_component_indices.push(i);
        }

        //push root component node into tree
        tree.nodes.push(ComponentNode::new(tree.components.len() - 1, 0, &available_component_indices));

        //build tree
        let mut exploration_queue = VecDeque::new();
        exploration_queue.push_back(0);

        while !exploration_queue.is_empty() {
            //get the data from the node we want to explore
            let node_index = exploration_queue.pop_front().unwrap();
            let available_port = tree.nodes[node_index].available_port;
            let available_component_indices = tree.nodes[node_index].available_component_indices.clone();

            //for each of the explore node's available components
            for component_index in available_component_indices.iter() {
                let component = tree.components[*component_index].clone();

                //if we can attach this available component to the exploration node
                if component.0 == available_port ||
                   component.1 == available_port {
                       let new_node_available_port = if component.0 == available_port {
                           component.1
                       } else {
                           component.0
                       };
                       
                       //push a new node to the tree with this available component added
                       tree.nodes.push(ComponentNode::new(*component_index, new_node_available_port, &available_component_indices));
                       
                       //push this new node to the exploration stack so we can explore it later
                       exploration_queue.push_back(tree.nodes.len() - 1);
                       
                       //add this new node as a child to the node we're currently exploring
                       tree.nodes[node_index].add_child(exploration_queue.back().unwrap().clone());
                }
            }
        }

        tree
    }

    pub fn find_strongest_bridge(&self, node: usize) -> u32 {
        let node = &self.nodes[node];
        let component = self.components[node.component_index].clone();
        let component_strength = component.0 + component.1;

        let mut strongest_child = 0;
        for child in node.children_indices.iter() {
            let child_strength = self.find_strongest_bridge(*child);

            if (component_strength + child_strength) > (component_strength + strongest_child) {
                strongest_child = child_strength;
            }
        }

        component_strength + strongest_child
    }
}