struct Node {
    value: u32,
    next: Option<usize>,
}

impl Node {
    fn new(value: u32) -> Node {
        Node {
            value,
            next: None,
        }
    }

    fn get_next(&self) -> Option<usize> {
        self.next
    }

    fn set_next(&mut self, next: Option<usize>) {
        self.next = next;
    }

    fn get_value(&self) -> u32 {
        self.value
    }
}

pub struct Spinlock {
    list: Vec<Node>,
}

impl Spinlock {
    pub fn new() -> Spinlock {
        Spinlock {
            list: Vec::new(),
        }
    }

    pub fn solve_spinlock(&mut self, steps: u32, stop_limit: u32) -> u32 {
        let stop_limit = stop_limit + 1;

        //add the first node
        self.list.push(Node::new(0));

        let mut current_node = 0;
        for i in 1..stop_limit {
            //step through the list for step count
            for _ in 0..steps {
                current_node = match self.list[current_node].get_next() {
                    Some(val) => val,
                    None => 0, //reset back to head node
                };
            }

            //construct new node
            let mut new_node = Node::new(i);

            //connect new node to the node current_node points to
            new_node.set_next(self.list[current_node].get_next());
            
            //push the new node onto the list
            self.list.push(new_node);

            //point the current node at the new_node
            let new_node_idx = Some(self.list.len() - 1);
            self.list[current_node].set_next(new_node_idx);

            //set current node to recently inserted new node
            current_node = new_node_idx.unwrap();
        }

        //return the node value after the current_node
        let next_node = match self.list[0].get_next() {
            Some(val) => val,
            None => 0,
        };

        let result = self.list[next_node].get_value();

        self.list.clear();

        result
    }

    pub fn solve_spinlock_fast(&self, steps: u32, stop_limit: u32) -> u32 {
        let stop_limit = stop_limit + 1;
        
        let mut value_after_zero = 0;
        let mut current_pos = 0;

        for i in 1..stop_limit {
            let mut steps_to_take = steps;

            if steps > i {
                steps_to_take = steps % i; 
            }

            current_pos += steps_to_take;

            if current_pos >= i {
                current_pos -= i;
            }

            if current_pos == 0 {
                value_after_zero = i;
            }

            current_pos += 1;
        }

        value_after_zero
    }
}