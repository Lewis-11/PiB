use std::collections::HashMap;
use crate::fasta::Alignment;

pub(crate) fn get_center_string(adjacency_matrix: &HashMap<(String, String), Alignment>, maximize: bool) -> &String {
    let mut strings_cumulative_score: HashMap<&String, i32> = HashMap::new();
    // iterate over every key in the adjacency matrix
    // check if the sum of all of its alignment scores is the highest or (or lowest if maximize=false)
    let mut max_score = if maximize { i32::MIN } else { i32::MAX };
    let mut max_string: &String = &adjacency_matrix.keys().next().unwrap().0; // placeholder for the max string (will be overwritten)
    for (key, value) in adjacency_matrix.iter() {
        let (seq1, _) = key;
        let score = value.score;
        let score_cumulative = strings_cumulative_score.entry(seq1).or_insert(0);
        *score_cumulative += score;
        if maximize && *score_cumulative > max_score {
            max_score = *score_cumulative;
            max_string = seq1;
        } else if !maximize && *score_cumulative < max_score {
            max_score = *score_cumulative;
            max_string = seq1;
        }
    }
    return max_string;
}

