use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub(crate) fn parse_submatrix_string(
    submatrix_string: String,
) -> HashMap<u8, HashMap<u8, i32>> {
    let mut submatrix = HashMap::new();
    let mut lines = submatrix_string.lines();
    // splits the first line removing ',' and store the rest into a vector of bytes
    let header = lines
        .next()
        .expect("[!] Error parsing submatrix: string content is empty")
        .to_uppercase().as_str()
        .split(',')
        .map(|x| x.as_bytes()[0])
        .collect::<Vec<u8>>();

    for (i, line) in lines.enumerate() {
        let h = header[i].clone();
        // splits the first line removing ',' and store the rest into a vector of i32
        let cost = line
            .split(',')
            .map(|x| x.trim().to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut hm = HashMap::new();
        for (j, c) in cost.iter().enumerate() {
            hm.insert(header[j].clone(), *c);
        }
        submatrix.insert(h, hm);
    }
    return submatrix;
}

// Parse submatrix indicating cost of subtitution for each pair of characters.
// Returns a hashmap of the form: [char1][char2] -> cost.
pub(crate) fn read_submatrix_file(filename: &str) -> HashMap<u8, HashMap<u8, i32>> {
    let mut file = File::open(filename).expect("[!] Error parsing submatrix file: file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    return parse_submatrix_string(contents);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::utils::parse_submatrix_string;

    #[test]
    #[should_panic]
    fn test_empty_submatrix() {
        let empty_submatrix = String::new();
        parse_submatrix_string(empty_submatrix);
    }

    #[test]
    fn test_nonempty_submatrix() {
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
