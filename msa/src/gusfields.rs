
pub fn get_center_string(adjacency_matrix: &Vec<Vec<i32>>, maximize: bool) -> usize {
    let n = adjacency_matrix.len();
    let mut center_string = 0;
    let mut max_score = 0;

    for i in 0..n {
        let mut score = 0;
        for j in 0..n {
            score += &adjacency_matrix[i][j];
        }
        if maximize && (&score > &max_score) {
            max_score = score;
            center_string = i;
        } else if !maximize && (&score < &max_score) {
            max_score = score;
            center_string = i;
        }
    }
    return center_string;
}

fn insert_gap_at(matrix: &mut Vec<Vec<u8>>, index: usize) {
    for row in matrix.iter_mut() {
        row.insert(index, b'-');
    }
}

pub fn merge_clusters(str1: &String, str1_pair: &String, str2_pair: &String) -> Vec<i32> {
    let mut instructions: Vec<i32> = Vec::new();
    let str1: Vec<u8> = str1.as_bytes().to_vec();
    let str1_pair: Vec<u8> = str1_pair.as_bytes().to_vec();
    let str2_pair: Vec<u8> = str2_pair.as_bytes().to_vec();
    let mut i = 0;
    let mut j = 0;

    while i < str1.len() && j < str1_pair.len() {

        // Case 1:
        if str1[i] == str2_pair[j] {
            // add chars str1[i] and str2_pair[j] to result
            instructions.push(1);
            i += 1;
            j += 1;
        }

        // Case 2:
        else if str1[i] == b'-' && str1_pair[j] != b'-' {
            // add chars str1[i] and '-' instead of str2_pair[j]
            instructions.push(2);
            i += 1;
        }

        // Case 3:
        else if str1[i] != b'-' && str1_pair[j] == b'-' {
            // add str2_pair[j] and '-' instead of str[i]
            instructions.push(3);
            j += 1;
        }
    }
    while j < str1_pair.len() {
        instructions.push(3);
        j += 1;
    }
    while i < str1.len() {
        instructions.push(2);
        i += 1;
    }
    return instructions;
}

pub fn gusfield_mst(score_matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let center_string = get_center_string(score_matrix, true);
    let n = score_matrix.len();
    let mut merge_order : Vec<Vec<i32>>= Vec::new();
    for i in 0..n {
        if i == center_string { continue; };
        merge_order.push(vec![center_string as i32, i as i32]);
    }
    return merge_order;
}

