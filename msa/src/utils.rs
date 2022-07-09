use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// Parse submatrix indicating cost of subtitution for each pair of characters.
// Returns a hashmap of the form: [char1][char2] -> cost.
pub(crate) fn parse_submatrix(filename: &str) -> HashMap<char, HashMap<char, i32>> {
    let mut submatrix = HashMap::new();
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_string();
    let mut lines = contents.lines();
    // splits the first line removing ',' and store the rest into a vector of characters
    let header = lines.next().unwrap().split(',').map(|x| x.trim().to_string().chars().next().unwrap()).collect::<Vec<char>>();
    for (i, line) in lines.enumerate() {
        let h = header[i];
        // splits the first line removing ',' and store the rest into a vector of i32
        let cost = line.split(',').map(|x| x.trim().to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut hm = HashMap::new();
        for (j, c) in cost.iter().enumerate() {
            hm.insert(header[j], *c);
        }
        submatrix.insert(h, hm);
    }
    submatrix
}