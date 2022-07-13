use std::cmp::{max, min};
use std::collections::HashMap;
use crate::fasta::FastaSequence;

// Function to return the cost of aligning two fasta sequences.
// The substitution matrix is a hashmap of the form: [char1][char2] -> cost.
// The GAP_COST will be a constant of 5.
pub(crate) fn iterative_alignment_cost(seq1: &FastaSequence, seq2: &FastaSequence, submatrix: &HashMap<char, HashMap<char, i32>>, gap_cost: &i32, maximize: &bool) -> Vec<Vec<i32>> {
    let n = seq1.sequence.len() + 1;
    let m = seq2.sequence.len() + 1;

    let mut score_matrix = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {

            let (mut v1, mut v2, mut v3, mut v4): (i32, i32, i32, i32) =  if *maximize {
                (i32::MIN, i32::MIN, i32::MIN, i32::MIN)
            } else {
                (i32::MAX, i32::MAX, i32::MAX, i32::MAX)
            };
            if i>0 && j>0 {
                v1 = score_matrix[i-1][j-1] + submatrix[&(seq1.sequence.as_bytes()[i-1] as char)][&(seq2.sequence.as_bytes()[j-1] as char)];
            }
            if i>0 {
                v2 = score_matrix[i-1][j] + *gap_cost;
            }
            if j>0 {
                v3 = score_matrix[i][j-1] + *gap_cost;
            }
            if i==0 && j==0 {
                v4 = 0;
            }

            if *maximize {
                score_matrix[i][j] = max(v1, max(v2, max(v3, v4)));
            } else {
                score_matrix[i][j] = min(v1, min(v2, min(v3, v4)));
            }
        }
    }
    return score_matrix;
}

// Tests
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::fasta::parse_fasta_string;
    use crate::utils::parse_submatrix_string;
    use super::iterative_alignment_cost;

    #[test]
    fn test_sample_sequences() {
        let singleline_fasta = String::from_str(
            ">seq1\n\
            acgtgtcaacgt\n\
            >seq2\n\
            acgtcgtagcta\n\
            >seq3\n\
            aataat\n\
            >seq4\n\
            aagg\n\
            >seq5\n\
            tccagaga\n\
            >seq6\n\
            tcgat\n\
            >seq7\n\
            ggcctaaaggcgccggtctttcgtaccccaaaatctcggcattttaaga\
            taagtgagtgttgcgttacactagcgatctaccgcgtcttatacttaag\
            cgtatgcccagatctgactaatcgtgcccccggattagacgggcttgat\
            gggaaagaacagctcgtctgtttacgtataaacagaatcgcctgggttcgc\n\
            >seq8\n\
            gggctaaaggttagggtctttcacactaaagagtggtgcgtatcgtggc\
            taatgtaccgcttctggtatcgtggcttacggccagacctacaagtact\
            agacctgagaactaatcttgtcgagccttccattgagggtaatgggaga\
            gaacatcgagtcagaagttattcttgtttacgtagaatcgcctgggtccgc"
        ).unwrap();
        let result = parse_fasta_string(singleline_fasta);
        let seq1 = &result[0];
        let seq2 = &result[1];
        let seq3 = &result[2];
        let seq4 = &result[3];
        let seq5 = &result[4];
        let seq6 = &result[5];
        let seq7 = &result[6];
        let seq8 = &result[7];
        let submatrix = parse_submatrix_string(String::from_str(
            "A,C,G,T\n0,5,2,5\n5,0,5,2\n2,5,0,5\n5,2,5,0"
        ).unwrap());
        let mut score_matrix = iterative_alignment_cost(seq1, seq2, &submatrix, &5, &false);
        assert_eq!(score_matrix[seq1.sequence.len()][seq2.sequence.len()], 22);
        score_matrix = iterative_alignment_cost(seq3, seq4, &submatrix, &5, &false);
        assert_eq!(score_matrix[seq3.sequence.len()][seq4.sequence.len()], 14);
        score_matrix = iterative_alignment_cost(seq5, seq6, &submatrix, &5, &false);
        assert_eq!(score_matrix[seq5.sequence.len()][seq6.sequence.len()], 20);
        score_matrix = iterative_alignment_cost(seq7, seq8, &submatrix, &5, &false);
        assert_eq!(score_matrix[seq7.sequence.len()][seq8.sequence.len()], 325);
    }
}

