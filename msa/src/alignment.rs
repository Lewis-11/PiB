use std::cmp::{max, min};
use std::collections::HashMap;
use crate::fasta::FastaSequence;
use crate::fasta::Alignment;

// Function to return the cost of aligning two fasta sequences.
// The substitution matrix is a hashmap of the form: [char1][char2] -> cost.
pub(crate) fn iterative_pairwise_alignment_cost(seq1: &FastaSequence, seq2: &FastaSequence, sub_matrix: &HashMap<char, HashMap<char, i32>>, gap_cost: i32, maximize: bool) -> Option<Vec<Vec<i32>>> {

    if seq1.sequence.len() == 0 || seq2.sequence.len() == 0 || sub_matrix.len() == 0 {
        return None;
    }

    let n = seq1.sequence.len() + 1;
    let m = seq2.sequence.len() + 1;

    let seq1_bytes = seq1.sequence.as_bytes();
    let seq2_bytes = seq2.sequence.as_bytes();

    let mut score_matrix = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {

            let (mut v1, mut v2, mut v3, mut v4): (i32, i32, i32, i32) =  if maximize {
                (i32::MIN, i32::MIN, i32::MIN, i32::MIN)
            } else {
                (i32::MAX, i32::MAX, i32::MAX, i32::MAX)
            };
            if i>0 && j>0 {
                v1 = score_matrix[i-1][j-1] + sub_matrix[&(seq1_bytes[i-1] as char)][&(seq2_bytes[j-1] as char)];
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

            if maximize {
                score_matrix[i][j] = max(v1, max(v2, max(v3, v4)));
            } else {
                score_matrix[i][j] = min(v1, min(v2, min(v3, v4)));
            }
        }
    }
    return Some(score_matrix);
}

pub(crate) fn iterative_backtracking(score_matrix: &Vec<Vec<i32>>, seq1: &FastaSequence, seq2: &FastaSequence, sub_matrix: &HashMap<char, HashMap<char, i32>>, gap_cost: i32) -> Option<(FastaSequence, FastaSequence)> {

    if seq1.sequence.len() == 0 || seq2.sequence.len() == 0 || sub_matrix.len() == 0  || score_matrix.len() == 0 {
        return None;
    }

    let mut i = seq1.sequence.len();
    let mut j = seq2.sequence.len();

    let seq1_bytes = seq1.sequence.as_bytes();
    let seq2_bytes = seq2.sequence.as_bytes();

    let mut alignment1 = String::new();
    let mut alignment2 = String::new();


    while i>0 || j>0 {
        if i>0 && j>0 && score_matrix[i][j] == score_matrix[i-1][j-1] + sub_matrix[&(seq1_bytes[i-1] as char)][&(seq2_bytes[j-1] as char)] {
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

pub(crate) fn pairwise_alignment(seq1: &FastaSequence, seq2: &FastaSequence, sub_matrix: &HashMap<char, HashMap<char, i32>>, gap_cost: i32, maximize: bool) -> Option<Alignment> {
    let score_matrix: Vec<Vec<i32>> = iterative_pairwise_alignment_cost(seq1, seq2, sub_matrix, gap_cost, maximize)?;
    let (output1, output2) = iterative_backtracking(&score_matrix, seq1, seq2, sub_matrix, gap_cost)?;
    let score = score_matrix[seq1.sequence.len()][seq2.sequence.len()];
    return Some(Alignment::new(output1, output2, score));
}



// Tests
#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::fasta::parse_fasta_string;
    use crate::utils::parse_submatrix_string;
    use super::iterative_pairwise_alignment_cost;
    use super::iterative_backtracking;

    #[test]
    fn test_sample_sequences_alignment() {
        let single_line_fasta = String::from_str(
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
        let result = parse_fasta_string(single_line_fasta);
        let seq1 = &result[0];
        let seq2 = &result[1];
        let seq3 = &result[2];
        let seq4 = &result[3];
        let seq5 = &result[4];
        let seq6 = &result[5];
        let seq7 = &result[6];
        let seq8 = &result[7];
        let sub_matrix = parse_submatrix_string(String::from_str(
            "A,C,G,T\n0,5,2,5\n5,0,5,2\n2,5,0,5\n5,2,5,0"
        ).unwrap());

        let mut score_matrix = iterative_pairwise_alignment_cost(seq1, seq2, &sub_matrix, 5, false).unwrap();
        let mut alignment = iterative_backtracking(&score_matrix, seq1, seq2, &sub_matrix, 5);
        assert_eq!(score_matrix[seq1.sequence.len()][seq2.sequence.len()], 22);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("ACGT-GTCAACGT"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("ACGTCGT-AGCTA"));

        score_matrix = iterative_pairwise_alignment_cost(seq3, seq4, &sub_matrix, 5, false).unwrap();
        alignment = iterative_backtracking(&score_matrix, seq3, seq4, &sub_matrix, 5);
        assert_eq!(score_matrix[seq3.sequence.len()][seq4.sequence.len()], 14);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("AATAAT"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("AA-GG-"));

        score_matrix = iterative_pairwise_alignment_cost(seq5, seq6, &sub_matrix, 5, false).unwrap();
        alignment = iterative_backtracking(&score_matrix, seq5, seq6, &sub_matrix, 5);
        assert_eq!(score_matrix[seq5.sequence.len()][seq6.sequence.len()], 20);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("TCCAGAGA"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("T-C-GA-T"));

        score_matrix = iterative_pairwise_alignment_cost(seq7, seq8, &sub_matrix, 5, false).unwrap();
        alignment = iterative_backtracking(&score_matrix, seq7, seq8, &sub_matrix, 5);
        assert_eq!(score_matrix[seq7.sequence.len()][seq8.sequence.len()], 325);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("GGCCTAAAGGCGCCGGTCTTTCGTACCCCAAAATCTCG-GCATTTTAAGATAAGTG-AGTGTTGCGTTACACTAGCGATCTACCGCGTCTTATACT-TAAGCG-TATGCCC-AGATCTGA-CTAATCGTGCCCCCGGATTAGACGGGCTTGATGGGAAAGAACA--G-CTC-G--TCTGTTTACGTATAAACAGAATCGCCTGGGTTCGC"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("GGGCTAAAGGTTAGGGTCTTTCACACTAAAGAGTGGTGCGTATCGT-GGCTAA-TGTACCGCTTC-TGGTATC-GTGGCTTA-CG-GCCAGAC-CTACAAGTACTAGACCTGAGAACTAATCTTGTCGAGCCTTC-CATT-GA-GGG--TAATGGGAGAGAACATCGAGTCAGAAGTTATTCTTGTTTACGTAGAATCGCCTGGGTCCGC"));
    }
}

