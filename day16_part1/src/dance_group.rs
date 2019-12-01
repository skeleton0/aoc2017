pub struct DanceGroup {
    group: Vec<char>,
}

impl DanceGroup {
    pub fn new(group: Vec<char>) -> DanceGroup {
        DanceGroup {
            group,
        }
    }

    pub fn spin(&mut self, dancers: u32) {
        let mut new_group = Vec::new();

        let spin_index = self.group.len() - dancers as usize;

        //add specified ending dancers to the start
        for i in spin_index..self.group.len() {
            new_group.push(self.group[i]);
        }

        //add remaining dancers to the end
        for i in 0..spin_index {
            new_group.push(self.group[i]);
        }

        self.group = new_group;
    }

    pub fn exchange(&mut self, a: usize, b: usize) {
        let temp = self.group[a];

        self.group[a] = self.group[b];

        self.group[b] = temp;
    }

    pub fn partner(&mut self, a: char, b: char) {
        let mut a_index = 0;
        let mut b_index = 0;

        for (i, c) in self.group.iter().enumerate() {
            if *c == a {
                a_index = i;
            }

            if *c == b {
                b_index = i;
            }
        }

        self.exchange(a_index, b_index);
    }

    pub fn get_group(&self) -> &[char] {
        &self.group
    }
}