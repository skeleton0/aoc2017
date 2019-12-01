fn main() {
    println!("Enter captcha: ");

    let mut user_input = String::new();

    std::io::stdin().read_line(&mut user_input)
        .expect("Failed to read line.");

    println!("user_input.trim().len(): {}", user_input.trim().len());
    println!("user_input.len(): {}", user_input.len());

    let solution = captcha_sum(&user_input, 0);

    println!("Captcha sum: {}", solution);
}

fn captcha_sum(sequence: &String, index: usize) -> u32 {
    println!("index: {}", index);

    //get the appropriate next index
    let next_index = if index == sequence.trim().len() - 1 {
        0 
    } else {
        index + 1 
    };

    println!("next_index: {}", next_index);

    //get first value
    let first_val = if next_index != 0 {
        &sequence[index..next_index]
    } else {
        &sequence[index..].trim()
    };

    println!("first_val: {}", first_val);

    //get second value
    let second_val = if next_index == sequence.trim().len() - 1 {
        &sequence[next_index..].trim()
    } else {
        &sequence[next_index..next_index+1]
    };
    
    println!("second_val: {}", second_val);

    //get appropriate result
    let result: u32 = if first_val == second_val {
        first_val.parse().expect("Could not parse string.")
    } else {
        0
    };

    println!("result: {}", result);
    
    println!("");

    //return appropriate value
    if next_index == 0 {
        result
    } else {
        result + captcha_sum(&sequence, index + 1)
    }
}
