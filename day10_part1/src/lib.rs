pub fn knot_hash_256(lengths: &Vec<u32>) -> u32 {
    //init the list
    let mut hash_list: [u32; 256] = [0; 256];
    for i in 0..hash_list.len() {
        hash_list[i] = i as u32;
    }

    let mut current_index = 0;
    let mut skip_length = 0;

    for length in lengths {
        let length = *length;

        //construct vector of segment to reverse
        let mut segment = Vec::new();

        let mut segment_index = current_index;
        for _ in 0..length {
            segment.push(hash_list[segment_index]);
            
            segment_index += 1;
            if segment_index >= hash_list.len() {
                segment_index = 0;
            }
        }

        //reverse segment
        reverse_list(&mut segment);

        //apply the reversed segment
        let mut segment_index = current_index;
        for i in 0..segment.len() {
            hash_list[segment_index] = segment[i];
            
            segment_index += 1;
            if segment_index >= hash_list.len() {
                segment_index = 0;
            }
        }

        current_index = current_index + length as usize + skip_length;

        //apply wrap-around to index advance
        if current_index >= hash_list.len() {
            current_index = current_index - hash_list.len();
        }

        skip_length += 1;
    }

    hash_list[0] * hash_list[1]
}

fn reverse_list(list: &mut Vec<u32>) {
    let list_copy = list.clone();

    let mut decrement_index = list.len();

    for increment_index in 0..list.len() {
        decrement_index -= 1;
        list[increment_index] = list_copy[decrement_index];
    }
}