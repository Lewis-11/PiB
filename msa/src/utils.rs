use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::fasta::Sequence;

pub fn parse_submatrix_string(
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
pub fn read_submatrix_file(filename: &str) -> HashMap<u8, HashMap<u8, i32>> {
    let mut file = File::open(filename).expect("[!] Error parsing submatrix file: file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    return parse_submatrix_string(contents);
}

pub fn insert_gap_at(matrix: &mut Vec<Vec<u8>>, index: usize) {
    for row in matrix.iter_mut() {
        row.insert(index, b'-');
    }
}

// Function meant to be used to initially map sequence names to IDs 
pub fn map_seq_name_to_id(sequences: &Vec<Sequence>) -> HashMap<String, usize> {
    let mut seq_map = HashMap::new();
    
    for i in 0..sequences.len() {
        seq_map.insert(sequences[i].name.clone(), i);
    }
    
    return seq_map;
}

#[cfg(test)]
#[path ="./tests/utils.rs"]
mod tests;
