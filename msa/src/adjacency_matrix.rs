use std::collections::HashMap;
use crate::fasta::{Alignment, Sequence};
use crate::alignment::pairwise_alignment;

pub fn alignment_adjacency_matrix(sequences: &Vec<Sequence>, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32, maximize: bool) -> Option<Vec<Vec<Alignment>>> {
    let n = sequences.len();
    // initialize the adjacency matrix
    let mut adjacency_matrix = vec![vec![Alignment::new(vec![], 0); n]; n];
    for i in 0..n {
        for j in i+1..n {
            let (seq1, seq2) = (&sequences[i], &sequences[j]);
            let alignment = pairwise_alignment(seq1, seq2, sub_matrix, gap_cost, maximize)?;
            adjacency_matrix[i][j] = alignment.clone();
            adjacency_matrix[j][i] = Alignment::new_pairwise(alignment.sequences[1].clone(), alignment.sequences[0].clone(), alignment.score.clone());
        }
        adjacency_matrix[i][i] = Alignment::new_pairwise(sequences[i].clone(), sequences[i].clone(), 0);
    }
    return Some(adjacency_matrix);
}

pub fn u8_matrix_to_alignment(matrix: &Vec<Vec<u8>>, sequences: &Vec<Sequence>, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32) -> Option<Alignment> {
    let score = get_alignment_cost(matrix, sub_matrix, gap_cost);
    let mut output = Alignment::new(Vec::new(), score);
    let n = matrix.len();
    for i in 0..n {
        let str = String::from_utf8(matrix[i].clone()).ok()?;
        let seq = Sequence::new(sequences[i].name.clone(), str);
        output.sequences.push(seq);
    }

    Some(output)
}

fn get_alignment_cost(matrix: &Vec<Vec<u8>>, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32) -> i32{
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let mut cost = 0;
    for col in 0..n_cols {
        for r1 in 0..n_rows {
            for r2 in r1+1..n_rows {
                if matrix[r1][col] == b'-' && matrix[r2][col] == b'-' {
                    continue;
                }
                else if matrix[r1][col] == b'-' || matrix[r2][col] == b'-' {
                    cost += gap_cost;
                }
                else {
                    let c1 = matrix[r1][col];
                    let c2 = matrix[r2][col];
                    cost += sub_matrix[&c1][&c2];
                }

            }
        }
    }
    return cost;
}
