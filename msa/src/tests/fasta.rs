mod tests {
    use std::str::FromStr;

    use crate::fasta::parse_fasta_string;

    #[test]
    fn empty_fasta() {
        let empty_fasta = String::new();
        let result = parse_fasta_string(empty_fasta);
        assert_eq!(0, result.len());
    }

    #[test]
    fn singleline_fasta() {
        let singleline_fasta = String::from_str(
            ">s1\nAA\n>s2\nBBB\n>s3\nCCCC"
        ).unwrap();
        let result = parse_fasta_string(singleline_fasta);

        assert_eq!(3, result.len());
        assert_eq!("s1", result[0].name);
        assert_eq!("AA", result[0].sequence);
        assert_eq!("s2", result[1].name);
        assert_eq!("BBB", result[1].sequence);
        assert_eq!("s3", result[2].name);
        assert_eq!("CCCC", result[2].sequence);
    }

    #[test]
    fn multiline_fasta() {
        let multiline_fasta = String::from_str(
            ">s1\nAA\nAA\n>s2\nBBB\nBBB\n>s3\nCCCC\nCCCC"
        ).unwrap();
        let result = parse_fasta_string(multiline_fasta);

        assert_eq!(3, result.len());
        assert_eq!("s1", result[0].name);
        assert_eq!("AAAA", result[0].sequence);
        assert_eq!("s2", result[1].name);
        assert_eq!("BBBBBB", result[1].sequence);
        assert_eq!("s3", result[2].name);
        assert_eq!("CCCCCCCC", result[2].sequence);
    }
}
