use std::fs::File;
use std::io::{BufRead, BufReader};

// Fasta Sequence struct
pub(crate) struct FastaSequence {
    name: String,
    sequence: String,
}
// Function for printing the FastaSequence struct
impl std::fmt::Display for FastaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{{\nname: {}\nseq: {}\n}}", self.name, self.sequence);
    }
}

// Fasta file format parser that is able to identify:
// - skip the description lines tarting with ';'
// - the name of the sequence starting with '>'
// - the sequence itself
pub(crate) fn read_fasta(file_name: &str) -> Vec<FastaSequence> {

    let mut sequences = Vec::new();
    let mut sequence = FastaSequence {
        name: String::new(),
        sequence: String::new(),
    };
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut is_sequence = false;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
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
            sequence.sequence += &line;
        }
    }
    sequences.push(sequence);
    return sequences;
}