pub fn knot_hash_256_64(lengths: &mut Vec<u8>) -> String {
    //add the standard length suffix values to lengths
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    //init the list
    let mut sparse_hash: [u8; 256] = [0; 256];
    for i in 0..sparse_hash.len() {
        sparse_hash[i] = i as u8;
    }

    run_hash(&lengths, &mut sparse_hash);

    let dense_hash = sparse_to_dense(&sparse_hash);

    dense_to_hex(&dense_hash)
}

fn dense_to_hex(dense_hash: &[u8; 16]) -> String {
    let mut hex = String::new();
    
    for dec in dense_hash.iter() {
        hex.push_str(&format!("{:x}", dec));
    }

    hex
}

fn sparse_to_dense(sparse_hash: &[u8; 256]) -> [u8; 16] {
    let mut dense_hash = [0; 16];

    for i in 0..dense_hash.len() {
        let start_index = 0 + dense_hash.len() * i;
        let end_index = start_index + dense_hash.len();
        
        let mut block = 0;

        for j in start_index..end_index {
            block = block ^ sparse_hash[j];
        }

        dense_hash[i] = block;
    }

    dense_hash
}

fn reverse_list(list: &mut Vec<u8>) {
    let list_copy = list.clone();

    let mut decrement_index = list.len();

    for increment_index in 0..list.len() {
        decrement_index -= 1;
        list[increment_index] = list_copy[decrement_index];
    }
}

fn run_hash(lengths: &Vec<u8>, sparse_hash: &mut [u8; 256]) {
    let mut current_index = 0;
    let mut skip_length = 0;

    //run for 64 rounds
    for _ in 0..64 {
        for length in lengths {
            let length = *length;

            //construct vector of segment to reverse
            let mut segment = Vec::new();

            let mut segment_index = current_index;
            for _ in 0..length {
                segment.push(sparse_hash[segment_index]);

                segment_index += 1;
                if segment_index >= sparse_hash.len() {
                    segment_index = 0;
                }
            }

            //reverse segment
            reverse_list(&mut segment);

            //apply the reversed segment
            let mut segment_index = current_index;
            for i in 0..segment.len() {
                sparse_hash[segment_index] = segment[i];

                segment_index += 1;
                if segment_index >= sparse_hash.len() {
                    segment_index = 0;
                }
            }

            current_index = current_index + length as usize + skip_length;
            //apply wrap-around to index advance
            while current_index >= sparse_hash.len() {
                current_index = current_index - sparse_hash.len();
            }

            skip_length += 1;
        }
    }
}