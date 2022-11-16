use std::collections::HashMap;
use crate::fasta::{Alignment, Sequence};
use crate::alignment::pairwise_alignment;

struct AdjacencyMatrix {
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
    
    pub fn get(&self, i: usize, j: usize) -> i32 {
        let index = self.get_linear_index(i, j);
        return self.value[index].score;
    }
}

pub fn alignment_adjacency_matrix(
    sequences: &Vec<Sequence>,
    sub_matrix: &HashMap<u8, HashMap<u8, i32>>,
    seq_id_map: &HashMap<String, usize>,
    gap_cost: i32,
    maximize: bool
) -> Option<Vec<Vec<Alignment>>> {
    // We need to know how many sequences we have
    let n = sequences.len();
    // Initialize the adjacency matrix
    let mut adjacency_matrix = vec![vec![Alignment::new(vec![], 0); n]; n];

    // Compute each sequence
    for i in 0..n {
        // diff(i, j) == diff(j, i) so we can skip a few
        for j in i+1..n {
            // Retrieve the two sequences we are comparing
            let (seq1, seq2) = (&sequences[i], &sequences[j]);
            // Let's retrieve the associated ID
            let (seq1_id, seq2_id): (usize, usize);
            
            // Retrieve first ID
            match seq_id_map.get(&seq1.name) {
                Some(&id) => seq1_id = id,
                _ => panic!("{} has not been mapped to an ID", seq1.name)
            }

            // Retrieve second ID
            match seq_id_map.get(&seq2.name) {
                Some(&id) => seq2_id = id,
                _ => panic!("{} has not been mapped to an ID", seq2.name)
            }
            
            // In this case we would also be able to just use i and j 
            // seq1_id = i;
            // seq2_id = j;
            
            // Compute alignment
            let alignment = pairwise_alignment(seq1, seq2, sub_matrix, gap_cost, maximize)?;
            
            // Store results in the matrix using the mapped IDs
            adjacency_matrix[seq1_id][seq2_id] = alignment.clone();
            adjacency_matrix[seq2_id][seq1_id] = Alignment::new_pairwise(
                alignment.sequences[1].clone(),
                alignment.sequences[0].clone(),
                alignment.score.clone()
            );
        }
        // Do we actually need this ?
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

#[cfg(test)]
#[path ="./tests/adjacency_matrix.rs"]
mod tests;
