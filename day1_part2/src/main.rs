fn main() {
    println!("Enter captcha to solve:");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line.");

    let solution = captcha_sum(&input.trim(), 0);

    println!("Solution: {}", solution);
}

fn captcha_sum(list: &str, index: usize) -> u32 {
    let list_length = list.len();

    //get first string slice
    let first_slice = if index == list_length - 1 {
        &list[index..]
    } else {
        &list [index..index+1]
    };

    //calculate second index
    let offset = list_length / 2;

    let second_index = if index + offset > list_length - 1 {
        index + offset - list_length
    } else {
        index + offset
    };

    //get second string slice
    let second_slice = if second_index == list_length - 1 {
        &list[second_index..]
    } else {
        &list[second_index..second_index+1]
    };

    //compare the slices and retrieve the result
    let result: u32 = if first_slice == second_slice {
        first_slice.parse().expect("Failed parsing.")
    } else {
        0
    };

    if index == list_length - 1 {
        result
    } else {
        result + captcha_sum(&list, index+1)
    }
}
