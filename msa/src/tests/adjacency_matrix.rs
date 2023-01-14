mod tests {
    use std::str::FromStr;
    use crate::{utils::parse_submatrix_string, adjacency_matrix::{get_alignment_cost, AdjacencyMatrix}, fasta::{FastaSequence, Alignment}};
    
    const MIN_BOUND: usize = 2;
    const MAX_BOUND: usize = 50;

    #[test]
    fn check_mirrored_data() {

        for n in MIN_BOUND..MAX_BOUND {
            let mut adjacency_matrix = AdjacencyMatrix::new(n);
            
            // Insert all multiple fake aligments to the matrix 
            for i in 0..adjacency_matrix.size {

                let new_seq_1 = FastaSequence::new(String::from("NAME1"), String::from("SEQUENCE1"));
                let new_seq_2 = FastaSequence::new(String::from("NAME1"), String::from("SEQUENCE1"));
                let new_score: i32 = (i * adjacency_matrix.size + i) as i32;
                let alignment = Alignment::new([new_seq_1, new_seq_2].to_vec(), new_score);
                adjacency_matrix.set(i, i, alignment);

                for j in i+1..adjacency_matrix.size {
                    let new_seq_1 = FastaSequence::new(String::from("NAME1"), String::from("SEQUENCE1"));
                    let new_seq_2 = FastaSequence::new(String::from("NAME2"), String::from("SEQUENCE2"));
                    let new_score: i32 = (i * adjacency_matrix.size + j) as i32;
                    let alignment = Alignment::new([new_seq_1, new_seq_2].to_vec(), new_score);
                    adjacency_matrix.set(i, j, alignment);
                }
            }
            
            // Check that the matrix is indeed mirrored 
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(adjacency_matrix.get_score(i, j), adjacency_matrix.get_score(j, i));
                    assert_eq!(adjacency_matrix.get_sequence(i, j, 0).sequence, adjacency_matrix.get_sequence(j, i, 1).sequence);
                    if i < j {
                        assert_eq!(adjacency_matrix.get_sequence(i, j, 0).sequence, String::from("SEQUENCE1"));
                        assert_eq!(adjacency_matrix.get_sequence(i, j, 1).sequence, String::from("SEQUENCE2"));
                    } else if i == j {
                        assert_eq!(adjacency_matrix.get_sequence(i, j, 0).sequence, String::from("SEQUENCE1"));
                        assert_eq!(adjacency_matrix.get_sequence(i, j, 1).sequence, String::from("SEQUENCE1"));
                    } else {
                        assert_eq!(adjacency_matrix.get_sequence(i, j, 0).sequence, String::from("SEQUENCE2"));
                        assert_eq!(adjacency_matrix.get_sequence(i, j, 1).sequence, String::from("SEQUENCE1"));
                    }
                    assert_eq!(adjacency_matrix.get_sequence(i, j, 1).sequence, adjacency_matrix.get_sequence(j, i, 0).sequence);
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


    #[test]
    fn cost_of_differing_alignments() {
        let submatrix_string =
            String::from_str("A,C,G,T\n0,5,2,5\n5,0,5,2\n2,5,0,5\n5,2,5,0").unwrap();
        let sub_matrix = parse_submatrix_string(&submatrix_string);
        let mut result: Vec<Vec<u8>> = Vec::new();
        
        result.push("----TGCTAGCTAGTCGACTCGATCGCATGCTCCAGCTAGCATCAGTCAGCATCTATCACGACTAC".as_bytes().to_vec());
        result.push("AGCATGCTAGCTA--CGACTCGATC--------CGATCAGCATACGCATCGATCAGCATCGTCAGC".as_bytes().to_vec());
        result.push("AGCATGCTAGCTAGTCG--TCGATCGCATGCTCACGTCAGCTAGCATCAGCTAGCATCAGCATCGA".as_bytes().to_vec());
        result.push("AGCATGCTA--TAGTCGACTCGATCGCATGCTCCGTCAGCATCGATCAGCATCGATCAGCATCAGT".as_bytes().to_vec());
        result.push("AGCATGCTAGCTA--CGACTCGATGGCATGCTCACGTCAGCATCGTATCAGCATCAGTCAGCATCA".as_bytes().to_vec());
        
        // Got result from an external program
        assert_eq!(1372, get_alignment_cost(&result, &sub_matrix, 5));
    }

    #[test]
    fn cost_of_completely_different_alignments() {
        let submatrix_string =
            String::from_str("T,C,G,A\n0,1,1,1\n1,0,1,1\n1,1,0,1\n1,1,1,0").unwrap();
        let sub_matrix = parse_submatrix_string(&submatrix_string);
        let mut result: Vec<Vec<u8>> = Vec::new();

        // Insert 3 alignments that are completely different
        for c in [b'T', b'G', b'C', b'A'].iter() {
            result.push(Vec::new());
            for _ in 0..25 {
                result.last_mut().unwrap().push(*c);
            }
        }
        
        assert_eq!(6 * 25, get_alignment_cost(&result, &sub_matrix, 1));
    }

    #[test]
    fn cost_of_equal_alignments() {
        let submatrix_string =
            String::from_str("T,C,G,A\n0,1,1,1\n1,0,1,1\n1,1,0,1\n1,1,1,0").unwrap();
        let sub_matrix = parse_submatrix_string(&submatrix_string);
        let mut result: Vec<Vec<u8>> = Vec::new();
        
        // Insert 3 alignments that are identical
        for i in 0..3 {
            result.push(Vec::new());
            for _ in 0..25 {
                result[i].push(b'T');
            }
        }
        
        assert_eq!(0, get_alignment_cost(&result, &sub_matrix, 1));
    }
}