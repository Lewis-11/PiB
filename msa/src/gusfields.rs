
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

pub fn merge_clusters(cl1_representative: Vec<u8>, cl2_representative: Vec<u8>, pairwise1: Vec<u8>, pairwise2: Vec<u8>) -> Vec<i32> {
    // merge cluster 1 and cluster2.
    // index1 is the index of the representative sequence of cluster1
    // index2 is the index of the representative sequence of cluster2
    // pairwise1 and pairwise2 are the pairwise alignments between the representative sequences of cluster1 and cluster2
    let pairwise_length = pairwise1.len();

    let mut i = 0; // index of the representative sequence in cluster1
    let mut j = 0; // index of the representative sequence in cluster2
    let mut k = 0; // index of pairwise

    let mut instructions: Vec<i32> = Vec::new();

    while i < cl1_representative.len() && j < cl2_representative.len() && k < pairwise_length {
        if cl1_representative[i] == pairwise1[k] {
            if cl2_representative[j] == pairwise2[k] {
                instructions.push(0);
                i += 1;
                j += 1;
                k += 1;
            } else if cl2_representative[j] == b'-' && pairwise2[k] != b'-' {
                instructions.push(1);
                j += 1;
            } else if cl2_representative[j] != b'-' && pairwise2[k] == b'-' {
                instructions.push(2);
                i += 1;
                k += 1;
            }
        } else if cl1_representative[i] == b'-' && pairwise1[k] != b'-' {
            if cl2_representative[j] == pairwise2[k] {
                instructions.push(3);
                i += 1;
            } else if cl2_representative[j] == b'-' && pairwise2[k] != b'-' {
                instructions.push(4);
                i += 1;
                j += 1;
            } else if cl2_representative[j] != b'-' && pairwise2[k] == b'-' {
                instructions.push(3);
                i += 1;
            }
        }
        else if cl1_representative[i] != b'-' && pairwise1[k] == b'-' {
            if cl2_representative[j] == pairwise2[k] {
                instructions.push(5);
                j += 1;
                k += 1;
            } else if cl2_representative[j] == b'-' && pairwise2[k] != b'-' {
                instructions.push(1);
                j += 1;
            } else if cl2_representative[j] != b'-' && pairwise2[k] == b'-' {
                instructions.push(6);
                k += 1;
            }
        }
    }

    while i < cl1_representative.len() {
        instructions.push(3);
        i += 1;
    }

    while j < cl2_representative.len() {
        instructions.push(1);
        j += 1;
    }
    return instructions;
}



    pub fn merge_clusters_last(str1: &String, str1_pair: &String, str2_pair: &String) -> Vec<i32> {
    let mut instructions: Vec<i32> = Vec::new();
    let str1: Vec<u8> = str1.as_bytes().to_vec(); // representative string of cluster 1
    let str1_pair: Vec<u8> = str1_pair.as_bytes().to_vec(); // pairwise alignment of string 1 with respect to string 2
    let str2_pair: Vec<u8> = str2_pair.as_bytes().to_vec(); // pairwise alignment of string 2 with respect to string 1
    let mut i = 0;
    let mut j = 0;

    while i < str1.len() && j < str1_pair.len() {

        // Case 1:
        if str1[i] == str2_pair[j] {
            //
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

