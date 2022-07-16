use std::collections::HashMap;
use crate::fasta::{Alignment, FastaSequence};
use crate::alignment::pairwise_alignment;

pub(crate) fn alignment_adjacency_matrix(sequences: Vec<FastaSequence>, sub_matrix: &HashMap<char, HashMap<char, i32>>, gap_cost: i32, maximize: bool) -> Option<HashMap<(String, String), Alignment>> {
    let mut adjacency_matrix = HashMap::new();
    let n = sequences.len();
    for i in 0..n {
        for j in i+1..n {
            let (seq1, seq2) = (&sequences[i], &sequences[j]);
            let alignment = pairwise_alignment(seq1, seq2, sub_matrix, gap_cost, maximize)?;
            adjacency_matrix.insert((seq1.name.clone(), seq2.name.clone()), alignment.clone());
            adjacency_matrix.insert((seq2.name.clone(), seq1.name.clone()), alignment.clone());
        }
    }
    return Some(adjacency_matrix);
}
