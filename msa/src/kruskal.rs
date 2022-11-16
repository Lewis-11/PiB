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

pub fn get_kruskal_order(adjacency_matrix: Vec<Vec<Alignment>>, seq_id_map: &HashMap<String, usize>) -> Vec<MergingStep> {
    
    let number_sequences = adjacency_matrix.len();
    let mut merging_order: Vec<(usize, usize)> = Vec::new();
    let mut alignments: BinaryHeap<&Alignment> = BinaryHeap::new();
    
    for i in 0..number_sequences {
        for j in i..number_sequences {
            alignments.push(&adjacency_matrix[i][j]);
        }
    }
    
    // FIXME: Not optimized at all
    let ordered_alignments: Vec<&Alignment> = alignments.into_sorted_vec();
    
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
    for i in number_sequences-1 .. 0 {
        
        let (seq1_name, seq2_name)
            = (&ordered_alignments[i].sequences[0].name, &ordered_alignments[i].sequences[1].name);

        let (seq1_id, seq2_id): (usize, usize);

        // Retrieve first ID
        match seq_id_map.get(&ordered_alignments[i].sequences[0].name) {
            Some(&id) => seq1_id = id,
            _ => panic!("{} has not been mapped to an ID", seq1_name)
        }

        // Retrieve second ID
        match seq_id_map.get(&ordered_alignments[i].sequences[1].name) {
            Some(&id) => seq2_id = id,
            _ => panic!("{} has not been mapped to an ID", seq2_name)
        }

        // FIXME: CHANGE !!!
        let first_group_idx = set_groups[&seq1_id];
        let second_group_idx = set_groups[&seq2_id];

        if !set_pool[first_group_idx].contains(&seq2_id) {
            // Look into split_at_mut to improve performance
            let added_group = set_pool[second_group_idx].clone();
            let parent_group = & mut set_pool[first_group_idx];
            parent_group.extend(added_group.iter());
            set_groups.insert(seq2_id, first_group_idx);
            merging_order.push((seq1_id, seq2_id));
        }
    }
    
    return Vec::new()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::fasta::{Alignment, Sequence};

    use super::get_kruskal_order;


    #[test]
    fn test_order() {
        const NAME_LENGTH: usize = 8;
        const VALUE_LENGTH: usize = 20;

        let mut adjacency_matrix = vec![vec![Alignment::new(vec![], 0); 5]; 5];
        
        for i in 0 .. 5 {
            for j in i+1 .. 5 {
                let new_seq_1 = Sequence::new_random(NAME_LENGTH, VALUE_LENGTH);
                let new_seq_2 = Sequence::new_random(NAME_LENGTH, VALUE_LENGTH);
                let new_alignment = Alignment::new([new_seq_1, new_seq_2].to_vec(), (i * 5 + j) as i32);

                adjacency_matrix[i][j] = new_alignment;
            }
        }
        
        get_kruskal_order(adjacency_matrix, &HashMap::new());
    }

}