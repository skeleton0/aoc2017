struct Scanner {
    range: u32,
    pos: u32,
    moving_forward: bool,
}

impl Scanner {
    fn new(range: u32) -> Scanner {
        if range <= 1 {
            panic!("Scanner must have a range larger than 1.");
        }

        Scanner {
            range,
            pos: 0,
            moving_forward: true,
        }
    }

    fn tick(&mut self) {
        if self.moving_forward {
            if self.pos + 1 >= self.range {
                self.moving_forward = false;
                self.pos -= 1;
            } 
            else {
                self.pos += 1;
            }
        } 
        else {
            if self.pos == 0 {
                self.moving_forward = true;
                self.pos += 1;
            }
            else {
                self.pos -= 1;
            }
        }
    }
}

pub struct Firewall {
    layers: Vec<Option<Scanner>>,
}

impl Firewall {
    pub fn new() -> Firewall {
        Firewall {
            layers: Vec::new(),
        }
    }

    pub fn add_scanner(&mut self, layer: usize, range: u32) {
        //fill in absent layers so we can add this scanner at specified layer
        while self.layers.len() < layer {
            self.layers.push(None);
        }

        self.layers.push(Some(Scanner::new(range)));
    }

    pub fn run_simulation(&mut self) -> u32 {
        let mut trip_severity = 0;

        //run the simulation
        for picosecond in 0..self.layers.len() {
            match &self.layers[picosecond] {
                &Some(ref scanner) => {
                    if scanner.pos == 0 {
                        trip_severity += picosecond as u32 * scanner.range;
                    }
                },
                &None => (),
            }

            //tick each scanner
            for scanner in self.layers.iter_mut() {
                match scanner {
                    &mut Some(ref mut s) => s.tick(),
                    &mut None => (),
                }
            }
        }

        //reset the scanners for successive simulations
        for layer in self.layers.iter_mut() {
            match layer {
                &mut Some(ref mut s) => {
                    s.pos = 0;
                    s.moving_forward = true;
                },
                _ => (),
            }
        }

        trip_severity
    }

    //true means escape was successful
    fn check_delay_outcome(&self, delay: u32) -> bool {
        for layer in 0..self.layers.len() {
            if let &Some(ref scanner) = &self.layers[layer] {
                let scanner_pos = (layer as u32 + delay) % ((scanner.range * 2) - 2);
                //if scanner is in start position when the packet gets there
                if scanner_pos == 0 {
                    return false;
                }
            }
        }

        true
    }

    pub fn delay_for_escape(&self) -> u32 {
        let mut delay = 0;

        while !self.check_delay_outcome(delay) {
            delay += 1;
        }

        delay        
    }
}