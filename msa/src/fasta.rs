use std::sync::atomic;
use std::{fs::File, cmp::Ordering};
use std::io::Read;

use serde::{Serialize, Deserialize};

/**********************************************************************
 * Struct Definition
 */
// Fasta Sequence struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alignment {
    pub sequences: Vec<Sequence>,
    pub score: i32,
}

/**********************************************************************
 * Struct Implementation
 */

// Function for creating a Sequence struct
impl Sequence {
    pub fn new(name: String, value: String) -> Sequence {
        return Sequence {
            name,
            value
        }
    }
}

// Function for printing the Sequence struct
impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Print in the the name and the sequence splitted in lines of 64 characters
        write!(f, ">{}\n", self.name)?;
        for line in self.value.as_bytes().chunks(64) {
            write!(f, "{}", String::from_utf8_lossy(&line))?;
            write!(f, "\n")?;
        }
        return Ok(());
    }
}

// Function for printing the Alignment struct
impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "; score of alignment: {}\n", self.score)?;
        for seq in &self.sequences {
            write!(f, "{}\n", seq)?;
        }
        return Ok(());
    }
}

impl PartialEq for Alignment {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Alignment {}

impl Ord for Alignment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Alignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Function for creating new Alignment struct
impl Alignment {
    pub fn new_pairwise(seq1: Sequence, seq2: Sequence, score: i32) -> Alignment {
        return Alignment {
            sequences: vec![seq1, seq2],
            score,
        };
    }
    // Function for creating a new Alignment struct from a vector of Sequence structs
    pub fn new(sequences: Vec<Sequence>, score: i32) -> Alignment {
        return Alignment {
            sequences,
            score,
        };
    }

}

/**********************************************************************
 * Functions
 */
pub fn parse_fasta_string(fasta_string: String) -> Vec<Sequence> {
    let mut sequences = Vec::new();
    let mut sequence = Sequence::new(String::new(), String::new());
    let mut is_sequence = false;

    for line in fasta_string.lines() {
        if line.starts_with('>') {
            if is_sequence {
                sequences.push(sequence);
                sequence = Sequence::new(String::new(), String::new());
            }
            sequence.name = line[1..].to_string();
            is_sequence = true;
        } else if line.starts_with(';') {
            is_sequence = false;
        } else if is_sequence {
            // first convert the sequence to uppercase
            let mut line_upper = line.to_string();
            line_upper.make_ascii_uppercase();
            sequence.value.push_str(&line_upper);
        }
    }
    if sequence.value.len() > 0 {
        sequences.push(sequence);
    }
    return sequences;
}

// Fasta file format parser that is able to identify:
// - skip the description lines tarting with ';'
// - the name of the sequence starting with '>'
// - the sequence itself
pub fn read_fasta_file(file_name: &str) -> Vec<Sequence> {
    let mut file = File::open(file_name).expect("[!] Error parsing fasta file: file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return parse_fasta_string(contents);
}

#[cfg(test)]
#[path ="./tests/fasta.rs"]
mod tests;
