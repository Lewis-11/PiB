use std::fs::File;
use std::io::Read;

// Fasta Sequence struct
#[derive(Debug)]
pub(crate) struct FastaSequence {
    pub(crate) name: String,
    pub(crate) sequence: String,
}
// Function for printing the FastaSequence struct
impl std::fmt::Display for FastaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{{\nname: {}\nseq: {}\n}}", self.name, self.sequence);
    }
}

pub(crate) fn parse_fasta_string(fasta_string: String) -> Vec<FastaSequence> {
    let mut sequences = Vec::new();
    let mut sequence = FastaSequence {
        name: String::new(),
        sequence: String::new(),
    };
    let mut is_sequence = false;

    for line in fasta_string.lines() {
        if line.starts_with('>') {
            if is_sequence {
                sequences.push(sequence);
                sequence = FastaSequence {
                    name: String::new(),
                    sequence: String::new(),
                };
            }
            sequence.name = line[1..].to_string();
            is_sequence = true;
        } else if line.starts_with(';') {
            is_sequence = false;
        } else if is_sequence {
            // first convert the sequence to uppercase
            let mut line_upper = line.to_string();
            line_upper.make_ascii_uppercase();
            sequence.sequence.push_str(&line_upper);
        }
    }
    if sequence.sequence.len() > 0 {
        sequences.push(sequence);
    }
    return sequences;
}

// Fasta file format parser that is able to identify:
// - skip the description lines tarting with ';'
// - the name of the sequence starting with '>'
// - the sequence itself
pub(crate) fn read_fasta_file(file_name: &str) -> Vec<FastaSequence> {
    let mut file = File::open(file_name).expect("[!] Error parsing fasta file: file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return parse_fasta_string(contents);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::parse_fasta_string;

    #[test]
    fn test_empty_fasta() {
        let empty_fasta = String::new();
        let result = parse_fasta_string(empty_fasta);
        assert_eq!(0, result.len());
    }

    #[test]
    fn test_singleline_fasta() {
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
    fn test_multiline_fasta() {
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
