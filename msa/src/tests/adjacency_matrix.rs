mod tests {

    use crate::{adjacency_matrix::AdjacencyMatrix, fasta::{Alignment, Sequence}};

    const MIN_BOUND: usize = 2;
    const MAX_BOUND: usize = 50;

    const NAME_LENGTH: usize = 8;
    const VALUE_LENGTH: usize = 20;

    #[test]
    fn check_mirrored_data() {

        for n in MIN_BOUND..MAX_BOUND {
            let mut adjacency_matrix = AdjacencyMatrix::new(n);
            
            // Insert all multiple fake aligments to the matrix 
            for i in 0..adjacency_matrix.size {
                for j in i..adjacency_matrix.size {
                    let new_seq_1 = Sequence::new_random(NAME_LENGTH, VALUE_LENGTH);
                    let new_seq_2 = Sequence::new_random(NAME_LENGTH, VALUE_LENGTH);
                    let new_score: i32 = (i * adjacency_matrix.size + j) as i32;
                    let alignment = Alignment::new([new_seq_1, new_seq_2].to_vec(), new_score);
                    adjacency_matrix.set(i, j, alignment);
                }
            }
            
            // Check that the matrix is indeed mirrored 
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(adjacency_matrix.get(i, j), adjacency_matrix.get(j, i));
                }
            }
        }

    }
    
    #[test]
    fn check_continuous_indeces() {
        
        let mut curr_idx;
        
        for n in MIN_BOUND..MAX_BOUND {
            curr_idx = 0;
            for i in 0..n {
                for j in i..n {
                    // Check upper matrix
                    assert_eq!(curr_idx, AdjacencyMatrix::linear_index(i, j, n));
                    // Check lower matrix
                    assert_eq!(curr_idx, AdjacencyMatrix::linear_index(j, i, n));
                    curr_idx += 1;
                }
            }
        }
        
    }
}
