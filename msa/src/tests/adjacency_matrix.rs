mod tests {
    use std::str::FromStr;
    use crate::{utils::parse_submatrix_string, adjacency_matrix::get_alignment_cost};


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