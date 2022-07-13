use std::cmp::{max, min};
use std::collections::HashMap;
use crate::fasta::FastaSequence;

// Function to return the cost of aligning two fasta sequences.
// The substitution matrix is a hashmap of the form: [char1][char2] -> cost.
// The GAP_COST will be a constant of 5.
pub(crate) fn iterative_alignment_cost(seq1: &FastaSequence, seq2: &FastaSequence, submatrix: &HashMap<char, HashMap<char, i32>>, gap_cost: &i32, maximize: &bool) -> Vec<Vec<i32>> {
    let n = seq1.sequence.len() + 1;
    let m = seq2.sequence.len() + 1;

    let mut score_matrix = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {

            let (mut v1, mut v2, mut v3, mut v4): (i32, i32, i32, i32) =  if *maximize {
                (i32::MIN, i32::MIN, i32::MIN, i32::MIN)
            } else {
                (i32::MAX, i32::MAX, i32::MAX, i32::MAX)
            };
            if i>0 && j>0 {
                v1 = score_matrix[i-1][j-1] + submatrix[&(seq1.sequence.as_bytes()[i-1] as char)][&(seq2.sequence.as_bytes()[j-1] as char)];
            }
            if i>0 {
                v2 = score_matrix[i-1][j] + *gap_cost;
            }
            if j>0 {
                v3 = score_matrix[i][j-1] + *gap_cost;
            }
            if i==0 && j==0 {
                v4 = 0;
            }

            if *maximize {
                score_matrix[i][j] = max(v1, max(v2, max(v3, v4)));
            } else {
                score_matrix[i][j] = min(v1, min(v2, min(v3, v4)));
            }
        }
    }
    return score_matrix;
}

