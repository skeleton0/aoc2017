pub fn score_stream(stream: &str) -> u32 {
    let mut total_score = 0;
    let mut score_tier = 0;
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
                _   => ()
            }
        }
        else {
            match c {
                '{' => score_tier += 1,
                '}' => { total_score += score_tier; score_tier -= 1; },
                '<' => inside_garbage = true,
                '!' => ignore = true,
                _   => (),
            }
        }
    }

    total_score
}