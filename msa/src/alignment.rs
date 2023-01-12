use std::cmp::{max, min};
use std::collections::HashMap;
use crate::adjacency_matrix::{adjacency_matrix_scores, alignment_adjacency_matrix, get_alignment_cost};
use crate::fasta::{FastaSequence, parse_fasta_string};
use crate::fasta::Alignment;
use crate::gusfields::{gusfield_mst, kruskal_mst, merge_clusters};
use crate::utils::parse_submatrix_string;

// Function to return the cost of aligning two fasta sequences.
// The substitution matrix is a hashmap of the form: [char1][char2] -> cost.
pub fn iterative_pairwise_alignment_cost(
    seq1: &FastaSequence,
    seq2: &FastaSequence,
    sub_matrix: &HashMap<u8, HashMap<u8, i32>>,
    gap_cost: i32,
    maximize: bool
) -> Option<Vec<Vec<i32>>> {

    if seq1.sequence.len() == 0 || seq2.sequence.len() == 0 || sub_matrix.len() == 0 {
        return None;
    }

    let n = seq1.sequence.len() + 1;
    let m = seq2.sequence.len() + 1;

    let seq1_bytes = seq1.sequence.as_bytes();
    let seq2_bytes = seq2.sequence.as_bytes();

    let mut score_matrix = vec![vec![0; m]; n];

    let initial_values = if maximize {
        (i32::MIN, i32::MIN, i32::MIN, i32::MIN)
    } else {
        (i32::MAX, i32::MAX, i32::MAX, i32::MAX)
    };

    let min_max_fn : &dyn Fn(i32, i32) -> i32 = if maximize {
        &max
    } else {
        &min
    };

    for i in 0..n {
        for j in 0..m {

            let (mut v1, mut v2, mut v3, mut v4): (i32, i32, i32, i32) =  initial_values;

            if i>0 && j>0 {
                v1 = score_matrix[i-1][j-1] + sub_matrix[&(seq1_bytes[i-1])][&(seq2_bytes[j-1])];
            }
            if i>0 {
                v2 = score_matrix[i-1][j] + gap_cost;
            }
            if j>0 {
                v3 = score_matrix[i][j-1] + gap_cost;
            }
            if i==0 && j==0 {
                v4 = 0;
            }

            score_matrix[i][j] = min_max_fn(v1, min_max_fn(v2, min_max_fn(v3, v4)));

        }
    }
    return Some(score_matrix);
}

pub fn iterative_backtracking(
    score_matrix: &Vec<Vec<i32>>,
    seq1: &FastaSequence,
    seq2: &FastaSequence,
    sub_matrix: &HashMap<u8, HashMap<u8, i32>>,
    gap_cost: i32
) -> Option<(FastaSequence, FastaSequence)> {

    if seq1.sequence.len() == 0
        || seq2.sequence.len() == 0
        || sub_matrix.len() == 0
        || score_matrix.len() == 0
    {
        return None;
    }

    let mut i = seq1.sequence.len();
    let mut j = seq2.sequence.len();

    let seq1_bytes = seq1.sequence.as_bytes();
    let seq2_bytes = seq2.sequence.as_bytes();

    let mut alignment1 = String::new();
    let mut alignment2 = String::new();


    while i>0 || j>0 {
        if i>0 && j>0 && score_matrix[i][j] == score_matrix[i-1][j-1] + sub_matrix[&(seq1_bytes[i-1])][&(seq2_bytes[j-1])] {
            alignment1.push(seq1_bytes[i-1] as char);
            alignment2.push(seq2_bytes[j-1] as char);
            i-=1;
            j-=1;
        } else if j>0 && score_matrix[i][j-1] + gap_cost == score_matrix[i][j] {
            alignment1.push('-');
            alignment2.push(seq2_bytes[j-1] as char);
            j-=1;
        } else if i>0 && score_matrix[i-1][j] + gap_cost == score_matrix[i][j] {
            alignment1.push(seq1_bytes[i-1] as char);
            alignment2.push('-');
            i-=1;
        }
        if i==0 && j==0 {
            let output1 = FastaSequence::new(seq1.name.clone(), alignment1.chars().rev().collect::<String>());
            let output2 = FastaSequence::new(seq2.name.clone(), alignment2.chars().rev().collect::<String>());
            return Some((output1, output2));
        }
    }
    None
}

pub fn pairwise_alignment(seq1: &FastaSequence, seq2: &FastaSequence, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32, maximize: bool) -> Option<Alignment> {
    let score_matrix: Vec<Vec<i32>> = iterative_pairwise_alignment_cost(seq1, seq2, sub_matrix, gap_cost, maximize)?;
    let (output1, output2) = iterative_backtracking(&score_matrix, seq1, seq2, sub_matrix, gap_cost)?;
    let score = score_matrix[seq1.sequence.len()][seq2.sequence.len()];
    return Some(Alignment::new_pairwise(output1, output2, score));
}

pub fn msa(sub_matrix: &String, gap_cost: i32, maximize: bool, fasta: &String, algorithm: &String) -> Option<(Vec<Vec<Vec<Vec<u8>>>>, i32)> {

    let sm = parse_submatrix_string(sub_matrix);
    let fasta_sequences = parse_fasta_string(fasta);
    let adjacency_matrix = alignment_adjacency_matrix(&fasta_sequences, &sm, gap_cost, maximize).expect("Error creating adjacency matrix");
    let adjacency_matrix_scores = adjacency_matrix_scores(&adjacency_matrix);

    let guide_tree = match algorithm.as_str() {
        "gusfield" => gusfield_mst(&adjacency_matrix_scores),
        "kruskal" => kruskal_mst(&adjacency_matrix_scores),
        _ => None
    }.expect("Error creating mst");

    let mut clusters: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut lookup: Vec<(i32, i32)> = Vec::new();
    for i in 0..fasta_sequences.len() {
        clusters.push(vec![fasta_sequences[i].sequence.as_bytes().to_vec(); 1]);
        lookup.push((i as i32, 0));
    }

    let mut merge_steps: Vec<Vec<Vec<Vec<u8>>>> = Vec::new();

    // Iterate through the guide tree (vector of tuples f size 2) assigning each of them to a different variable
    for (orig, dest) in guide_tree {
        let (orig_cluster, orig_index) = lookup[orig as usize];
        let (dest_cluster, dest_index) = lookup[dest as usize];
        let orig_cluster_len = clusters[orig_cluster as usize].len() as i32;
        let pairwise1 = &adjacency_matrix[orig as usize][dest as usize].sequences[0].sequence;
        let pairwise2 = &adjacency_matrix[orig as usize][dest as usize].sequences[1].sequence;
        let cl3 = merge_clusters(
            &clusters[orig_cluster as usize],
            &clusters[dest_cluster as usize],
            orig_index,
            dest_index,
            pairwise1.as_bytes().to_vec(),
            pairwise2.as_bytes().to_vec()
        ).expect("Error merging clusters");
        let step = vec![clusters[orig_cluster as usize].clone(), clusters[dest_cluster as usize].clone(), cl3.clone()];
        clusters[orig_cluster as usize] = cl3.clone();
        lookup[orig as usize] = (orig_cluster, orig_index);
        lookup[dest as usize] = (orig_cluster, orig_cluster_len + dest_index);
        merge_steps.push(step);
    }
    let last_step_last_cluster = merge_steps.last().unwrap().last().unwrap();
    let score = get_alignment_cost(last_step_last_cluster, &sm, gap_cost);
    return Some((merge_steps, score));
}

#[cfg(test)]
#[path ="./tests/alignment.rs"]
mod tests;
