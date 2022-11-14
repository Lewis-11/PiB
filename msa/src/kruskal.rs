use std::collections::{BinaryHeap, HashSet, HashMap};
use serde::{Serialize, Deserialize};

use crate::fasta::Alignment;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MergingStep {
    cluster_1: u32,
    cluster_2: u32,
    cluster_3: u32
} 

pub fn get_kruskal_order(adjacency_matrix: Vec<Vec<Alignment>>) -> Vec<MergingStep> {
    
    let number_sequences = adjacency_matrix.len();
    let mut merging_order: Vec<(usize, usize)> = Vec::new();
    let mut alignments: BinaryHeap<Alignment> = BinaryHeap::new();
    
    for i in 0..number_sequences {
        for j in i..number_sequences {
            alignments.push(adjacency_matrix[i][j].clone());
        }
    }
    
    // FIXME: Not optimized at all
    let ordered_alignments: Vec<Alignment> = alignments.into_sorted_vec();
    
    let mut set_pool: Vec<_> = (0 .. number_sequences)
        .map(|_| HashSet::<usize>::new())
        .collect();
    let mut set_groups: HashMap<usize, usize>
        = HashMap::with_capacity(number_sequences);
    
    // Each sequence represents a cluster
    for i in 0..adjacency_matrix.len() {
        set_groups.insert(i, i);
    }
    
    // Go through the alignments in order and write down the steps
    for i in number_sequences-1..0 {
        /* let first_identifier = ordered_alignments[i].sequences[0].identifier; */
        /* let second_identifier = ordered_alignments[i].sequences[1].identifier; */
        // FIXME: CHANGE !!!
        let first_identifier = 1;
        let second_identifier = 2;
        let first_group_idx = set_groups[&first_identifier];
        let second_group_idx = set_groups[&second_identifier];
        if !set_pool[first_group_idx].contains(&second_identifier) {
            // Look into split_at_mut to improve performance
            let added_group = set_pool[second_group_idx].clone();
            let parent_group = & mut set_pool[first_group_idx];
            parent_group.extend(added_group.iter());
            set_groups.insert(second_identifier, first_group_idx);
            merging_order.push((first_identifier, second_identifier));
        }
    }
    
    return Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::fasta::Alignment;

    use super::get_kruskal_order;


    #[test]
    fn test_order() {
        let mut adjacency_matrix = vec![vec![Alignment::new(vec![], 0); 5]; 5];
        
        for i in 0..5 {
            for j in 0..5 {
                adjacency_matrix[i][j].score = (i * 5 + j) as i32;
            }
        }
        
        get_kruskal_order(adjacency_matrix);
    }

}