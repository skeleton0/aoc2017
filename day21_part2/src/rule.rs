pub struct Rule {
    //rotations
    pattern: Vec<bool>,
    pattern_90: Vec<bool>,
    pattern_180: Vec<bool>,
    pattern_270: Vec<bool>,

    //flipped rotations
    pattern_flipped: Vec<bool>,
    pattern_flipped_90: Vec<bool>,
    pattern_flipped_180: Vec<bool>,
    pattern_flipped_270: Vec<bool>,

    output_pattern: Vec<bool>,
}

impl Rule {
    pub fn new(pattern: Vec<bool>, output_pattern: Vec<bool>) -> Rule {
        let rotation_matrix;
        let flip_matrix;

        match pattern.len() {
            4 => {
                rotation_matrix = vec![2, 0, 3, 1];
                flip_matrix = vec![1, 0, 3, 2];  
            },
            9 => {
                rotation_matrix = vec![6, 3, 0, 7, 4, 1, 8, 5, 2];
                flip_matrix = vec![2, 1, 0, 5, 4, 3, 8, 7, 6];
            },
            _ => panic!("Unrecognised pattern length."),
        }

        let transformation = |source_pattern: &Vec<bool>, transform_matrix: &Vec<u32>| {
            let mut transformed_pattern = vec![false; source_pattern.len()];

            for (i, source) in transform_matrix.iter().enumerate() {
                transformed_pattern[i] = source_pattern[*source as usize];
            }

            transformed_pattern
        };

        //declare transformation vars
        let pattern_90 = transformation(&pattern, &rotation_matrix);
        let pattern_180 = transformation(&pattern_90, &rotation_matrix);
        let pattern_270 = transformation(&pattern_180, &rotation_matrix);
        let pattern_flipped = transformation(&pattern, &flip_matrix);
        let pattern_flipped_90 = transformation(&pattern_flipped, &rotation_matrix);
        let pattern_flipped_180 = transformation(&pattern_flipped_90, &rotation_matrix);
        let pattern_flipped_270 = transformation(&pattern_flipped_180, &rotation_matrix);

        Rule {
            pattern,
            pattern_90,
            pattern_180,
            pattern_270,

            pattern_flipped,
            pattern_flipped_90,
            pattern_flipped_180,
            pattern_flipped_270,

            output_pattern,
        }
    }

    pub fn get_output_pattern(&self) -> &[bool] {
        self.output_pattern.as_slice()
    }

    pub fn pattern_matches(&self, pattern: &[bool]) -> bool {
        if pattern == self.pattern.as_slice() ||
           pattern == self.pattern_90.as_slice() ||
           pattern == self.pattern_180.as_slice() ||
           pattern == self.pattern_270.as_slice() ||
           pattern == self.pattern_flipped.as_slice() ||
           pattern == self.pattern_flipped_90.as_slice() ||
           pattern == self.pattern_flipped_180.as_slice() ||
           pattern == self.pattern_flipped_270.as_slice() {
               true
               } else {
               false
               }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn wrong_pattern_size() {
        let pattern = vec![true, false, false, true, false];
        let output_pattern = vec![true, true, true, true, false, true, false, false];

        Rule::new(pattern, output_pattern);
    }

    #[test]
    fn pattern_does_match() {
        let pattern = vec![false, false, false, true];
        let output_pattern = vec![true, true, false, true, false, false, false, false, false];

        let new_rule = Rule::new(pattern, output_pattern);

        let test_pattern = vec![true, false, false, false];

        assert!(new_rule.pattern_matches(&test_pattern));
    }

    #[test]
    fn pattern_does_not_match() {
        let pattern = vec![false, false, false, true];
        let output_pattern = vec![true, true, false, true, false, false, false, false, false];

        let new_rule = Rule::new(pattern, output_pattern);

        let test_pattern = vec![true, false, false, true];

        assert!(!new_rule.pattern_matches(&test_pattern));
    }
}