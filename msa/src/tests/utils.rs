mod tests {
    use std::str::FromStr;

    use crate::utils::parse_submatrix_string;

    #[test]
    #[should_panic]
    fn empty_submatrix() {
        let empty_submatrix = String::new();
        parse_submatrix_string(empty_submatrix);
    }

    #[test]
    fn nonempty_submatrix() {
        let nonempty_submatrix =
            String::from_str("T,C,G,A\n0,12,2,5\n3,0,11,1\n55,3,0,9\n1,2,3,0").unwrap();
        let result = parse_submatrix_string(nonempty_submatrix);

        assert_eq!(4, result.len());

        assert_eq!(4, result[&b'T'].len());
        assert_eq!(4, result[&b'C'].len());
        assert_eq!(4, result[&b'G'].len());
        assert_eq!(4, result[&b'A'].len());

        assert_eq!(0, result[&b'T'][&b'T']);
        assert_eq!(12, result[&b'T'][&b'C']);
        assert_eq!(2, result[&b'T'][&b'G']);
        assert_eq!(5, result[&b'T'][&b'A']);

        assert_eq!(3, result[&b'C'][&b'T']);
        assert_eq!(0, result[&b'C'][&b'C']);
        assert_eq!(11, result[&b'C'][&b'G']);
        assert_eq!(1, result[&b'C'][&b'A']);

        assert_eq!(55, result[&b'G'][&b'T']);
        assert_eq!(3, result[&b'G'][&b'C']);
        assert_eq!(0, result[&b'G'][&b'G']);
        assert_eq!(9, result[&b'G'][&b'A']);

        assert_eq!(1, result[&b'A'][&b'T']);
        assert_eq!(2, result[&b'A'][&b'C']);
        assert_eq!(3, result[&b'A'][&b'G']);
        assert_eq!(0, result[&b'A'][&b'A']);
    }
}
