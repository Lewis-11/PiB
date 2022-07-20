use std::collections::HashMap;
use crate::fasta::{Alignment, FastaSequence};

mod adjacency_matrix;
mod gusfields;

pub(crate) fn gusfield_msa(sequences: &Vec<FastaSequence>, sub_matrix: &HashMap<char, HashMap<char, i32>>, gap_cost: i32, maximize: bool) -> Option<Alignment> {
    let adjacency_matrix = adjacency_matrix::alignment_adjacency_matrix(sequences, sub_matrix, gap_cost, maximize)?;
    let alignment_matrix = gusfields::gusfield_alignment(&adjacency_matrix);
    return adjacency_matrix::u8_matrix_to_alignment(&alignment_matrix, sequences, sub_matrix, gap_cost);
}