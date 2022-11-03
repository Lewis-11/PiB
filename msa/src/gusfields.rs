use crate::fasta::Alignment;

pub fn get_center_string(adjacency_matrix: &Vec<Vec<i32>>, maximize: bool) -> usize {
    let n = adjacency_matrix.len();
    let mut center_string = 0;
    let mut max_score = 0;

    for i in 0..n {
        let mut score = 0;
        for j in 0..n {
            score += adjacency_matrix[i][j];
        }
        if maximize && score > max_score {
            max_score = score;
            center_string = i;
        } else if !maximize && score < max_score {
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

fn merge_clusters(mut cluster1: Vec<Vec<u8>>, mut cluster2: Vec<Vec<u8>>, cluster3: Vec<Vec<u8>>, mut idx1: usize, mut idx2: usize, mut idx3: usize, cluster1_string: usize) {

    // Case 1:
    if cluster1[cluster1_string][idx1] == cluster3[0][idx3] {
        idx1 += 1;
        idx2 += 1;
        idx3 += 1;
    }

    // Case 2:
    else if cluster1[cluster1_string][idx1] == b'-' && cluster3[0][idx3] != b'-' {
        insert_gap_at(&mut cluster2, idx2);
        idx1 += 1;
        idx2 += 1;
    }

    // Case 3:
    else if cluster1[cluster1_string][idx1] != b'-' && cluster3[0][idx3] == b'-' {
        insert_gap_at(&mut cluster1, i);
        idx1 += 1;
        idx2 += 1;
        idx3 += 1;
    }

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

// pub(crate) fn gusfield_alignment(adjacency_matrix: &Vec<Vec<Alignment>>) -> Vec<Vec<u8>> {
//     let n = adjacency_matrix.len();
//     let center_string = get_center_string(adjacency_matrix, true);
//     let mut matrix = Vec::new();
//     matrix.push(adjacency_matrix[center_string][center_string].sequences[0].sequence.as_bytes().to_vec());
//     for seq in 0..n {
//         if seq == center_string { continue };
//         let mut i:usize = 0;
//         let mut j:usize = 0;
//         let mut new_row = Vec::new();
//         let center_seq = adjacency_matrix[center_string][seq].sequences[0].sequence.as_bytes().to_vec();
//         let c_seq =      adjacency_matrix[seq][center_string].sequences[0].sequence.as_bytes().to_vec();
//         while i < matrix[0].len() && j < center_seq.len() {
//
//             // Case 1:
//             if matrix[0][i] == center_seq[j] {
//                 new_row.push(c_seq[j]);
//                 i += 1;
//                 j += 1;
//             }
//
//             // Case 2:
//             else if matrix[0][i] == b'-' && center_seq[j] != b'-' {
//                 new_row.push(b'-');
//                 i += 1;
//             }
//
//             // Case 3:
//             else if matrix[0][i] != b'-' && center_seq[j] == b'-' {
//                 insert_gap_at(&mut matrix, i);
//                 new_row.push(c_seq[j]);
//                 i += 1;
//                 j += 1;
//             }
//         }
//         while j < center_seq.len() {
//             insert_gap_at(&mut matrix, i);
//             new_row.push(c_seq[j]);
//             i += 1;
//             j += 1;
//         }
//         while i < matrix[0].len() {
//             new_row.push(b'-');
//             i += 1;
//         }
//         matrix.push(new_row);
//     }
//     matrix
// }



