use std::collections::HashMap;
use crate::fasta::{Alignment, FastaSequence};
use crate::alignment::pairwise_alignment;

pub struct AdjacencyMatrix {
    pub value: Vec<Alignment>,
    pub size: usize,
}

impl AdjacencyMatrix {
    pub fn new(size: usize) -> AdjacencyMatrix {
        return AdjacencyMatrix {
            // Triangular number
            value: vec![Alignment::new(vec![], 0); (size.pow(2) + size) / 2],
            size,
        }
    }
    
    // stackoverflow.com/questions/53233695/numpy-efficient-way-to-convert-indices-of-a-square-matrix-to-its-upper-triangul
    pub fn linear_index(i: usize, j: usize, n: usize) -> usize {
        let primary_idx = i.min(j);
        let secondary_idx = i.max(j);
        return (2 * n + 1 - primary_idx) * primary_idx / 2 + secondary_idx - primary_idx;
    }
    
    pub fn get_linear_index(&self, i: usize, j: usize) -> usize {
        return AdjacencyMatrix::linear_index(i, j, self.size);
    }
    
    pub fn set(&mut self, i: usize, j: usize, value: Alignment) {
        let index = self.get_linear_index(i, j);
        self.value[index] = value;
    }
    
    pub fn get_score(&self, i: usize, j: usize) -> i32 {
        let index = self.get_linear_index(i, j);
        return self.value[index].score;
    }
    
    pub fn get_sequence(&self, i: usize, j: usize, sequence_id: usize) -> &FastaSequence {
        let index = self.get_linear_index(i, j);
        let seq_id = if i > j { 1 - sequence_id } else { sequence_id };
        return &self.value[index].sequences[seq_id];
    }
}

pub fn alignment_adjacency_matrix(sequences: &Vec<FastaSequence>, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32, maximize: bool) -> Option<AdjacencyMatrix> {
    let n = sequences.len();
    // initialize the adjacency matrix
    // let mut adjacency_matrix = vec![vec![Alignment::new(vec![], 0); n]; n];
    let mut adjacency_matrix = AdjacencyMatrix::new(n);
    for i in 0..n {
        for j in i+1..n {
            let (seq1, seq2) = (&sequences[i], &sequences[j]);
            let alignment = pairwise_alignment(seq1, seq2, sub_matrix, gap_cost, maximize)?;
            adjacency_matrix.set(i, j, alignment.clone());
        }
        adjacency_matrix.set(i, i, Alignment::new_pairwise(sequences[i].clone(), sequences[i].clone(), 0));
    }
    return Some(adjacency_matrix);
}

pub fn adjacency_matrix_scores(alignment_matrix: &AdjacencyMatrix) -> Vec<Vec<i32>> {
    let n = alignment_matrix.size;
    let mut score_matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            score_matrix[i][j] = alignment_matrix.get_score(i, j);
        }
    }
    return score_matrix;
}

pub fn get_alignment_cost(matrix: &Vec<Vec<u8>>, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32) -> i32{
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

#[cfg(test)]
#[path ="./tests/adjacency_matrix.rs"]
mod tests;