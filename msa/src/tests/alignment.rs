mod tests {
    use std::str::FromStr;
    use crate::fasta::parse_fasta_string;
    use crate::utils::parse_submatrix_string;
    use crate::alignment::{iterative_pairwise_alignment_cost, iterative_backtracking};

    #[test]
    fn sample_sequences_alignment() {
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
        let result = parse_fasta_string(&single_line_fasta);
        let seq1 = &result[0];
        let seq2 = &result[1];
        let seq3 = &result[2];
        let seq4 = &result[3];
        let seq5 = &result[4];
        let seq6 = &result[5];
        let seq7 = &result[6];
        let seq8 = &result[7];
        let sub_matrix = parse_submatrix_string(&String::from_str(
            "A,C,G,T\n0,5,2,5\n5,0,5,2\n2,5,0,5\n5,2,5,0"
        ).unwrap());

        let mut score_matrix = iterative_pairwise_alignment_cost(seq1, seq2, &sub_matrix, 5, false).unwrap();
        let mut alignment = iterative_backtracking(&score_matrix, seq1, seq2, &sub_matrix, 5);
        assert_eq!(score_matrix[seq1.sequence.len()][seq2.sequence.len()], 22);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("ACGT-GTCAACGT"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("ACGTCGT-AGCTA"));
        
        score_matrix = iterative_pairwise_alignment_cost(seq2, seq1, &sub_matrix, 5, false).unwrap();
        assert_eq!(score_matrix[seq2.sequence.len()][seq1.sequence.len()], 22);

        score_matrix = iterative_pairwise_alignment_cost(seq3, seq4, &sub_matrix, 5, false).unwrap();
        alignment = iterative_backtracking(&score_matrix, seq3, seq4, &sub_matrix, 5);
        assert_eq!(score_matrix[seq3.sequence.len()][seq4.sequence.len()], 14);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("AATAAT"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("AA-GG-"));

        score_matrix = iterative_pairwise_alignment_cost(seq4, seq3, &sub_matrix, 5, false).unwrap();
        assert_eq!(score_matrix[seq4.sequence.len()][seq3.sequence.len()], 14);

        score_matrix = iterative_pairwise_alignment_cost(seq5, seq6, &sub_matrix, 5, false).unwrap();
        alignment = iterative_backtracking(&score_matrix, seq5, seq6, &sub_matrix, 5);
        assert_eq!(score_matrix[seq5.sequence.len()][seq6.sequence.len()], 20);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("TCCAGAGA"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("T-C-GA-T"));

        score_matrix = iterative_pairwise_alignment_cost(seq6, seq5, &sub_matrix, 5, false).unwrap();
        assert_eq!(score_matrix[seq6.sequence.len()][seq5.sequence.len()], 20);

        score_matrix = iterative_pairwise_alignment_cost(seq7, seq8, &sub_matrix, 5, false).unwrap();
        alignment = iterative_backtracking(&score_matrix, seq7, seq8, &sub_matrix, 5);
        assert_eq!(score_matrix[seq7.sequence.len()][seq8.sequence.len()], 325);
        assert_eq!(alignment.as_ref().unwrap().0.sequence, String::from("GGCCTAAAGGCGCCGGTCTTTCGTACCCCAAAATCTCG-GCATTTTAAGATAAGTG-AGTGTTGCGTTACACTAGCGATCTACCGCGTCTTATACT-TAAGCG-TATGCCC-AGATCTGA-CTAATCGTGCCCCCGGATTAGACGGGCTTGATGGGAAAGAACA--G-CTC-G--TCTGTTTACGTATAAACAGAATCGCCTGGGTTCGC"));
        assert_eq!(alignment.as_ref().unwrap().1.sequence, String::from("GGGCTAAAGGTTAGGGTCTTTCACACTAAAGAGTGGTGCGTATCGT-GGCTAA-TGTACCGCTTC-TGGTATC-GTGGCTTA-CG-GCCAGAC-CTACAAGTACTAGACCTGAGAACTAATCTTGTCGAGCCTTC-CATT-GA-GGG--TAATGGGAGAGAACATCGAGTCAGAAGTTATTCTTGTTTACGTAGAATCGCCTGGGTCCGC"));

        score_matrix = iterative_pairwise_alignment_cost(seq8, seq7, &sub_matrix, 5, false).unwrap();
        assert_eq!(score_matrix[seq8.sequence.len()][seq7.sequence.len()], 325);
    }
}

