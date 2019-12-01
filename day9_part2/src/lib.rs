pub fn count_garbage(stream: &str) -> u32 {
    let mut garbage_score = 0;
    let mut ignore = false;
    let mut inside_garbage = false;

    for c in stream.chars() {
        if ignore {
            ignore = false;
            continue;
        }

        if inside_garbage {
            match c {
                '>' => inside_garbage = false,
                '!' => ignore = true,
                _   => garbage_score += 1,
            }
        }
        else {
            match c {
                '<' => inside_garbage = true,
                '!' => ignore = true,
                _   => (),
            }
        }
    }

    garbage_score
}