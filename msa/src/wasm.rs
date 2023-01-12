use std::collections::HashMap;
use fasta::parse_fasta_string;
use utils::parse_submatrix_string;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use crate::alignment::msa;
use crate::fasta::Alignment;

mod fasta;
mod utils;
mod alignment;
mod adjacency_matrix;
mod gusfields;

// #[wasm_bindgen]
// pub fn wasm_serialize_fasta_string(fasta_string: String) -> JsValue {
//     let sequences = parse_fasta_string(fasta_string);
//     serde_wasm_bindgen::to_value(&sequences).unwrap_or(JsValue::NULL)
// }

// #[wasm_bindgen]
// pub fn wasm_serialize_submatrix(submatrix_string: String) -> JsValue {
//     let submatrix_cost = parse_submatrix_string(submatrix_string);
//     serde_wasm_bindgen::to_value(&submatrix_cost).unwrap_or(JsValue::NULL)
// }

/**
 * @param {Json} seq1 {name: string, sequence: string}
 * @param {Json} seq1 {name: string, sequence: string}
 * @param {HashMap [u8][u8] -> i32} sub_matrix
 * @param {number} gap_cost
 * @param {boolean} maximize
 * @returns {Alignment} {sequences: [FastaSequence, FastaSequence], score: number}
 */
// #[wasm_bindgen]
// pub fn wasm_pairwise_alignment(seq1: String, seq2: String, sub_matrix: String, gap_cost: i32, maximize: bool) -> JsValue {
//     let seq1 = serde_json::from_str(&seq1).unwrap();
//     let seq2 = serde_json::from_str(&seq2).unwrap();
//     let sub_matrix: HashMap<u8,HashMap<u8,i32>> = serde_json::from_str(&sub_matrix).unwrap();
//     let alignment = alignment::pairwise_alignment(&seq1, &seq2, &sub_matrix, gap_cost, maximize).unwrap_or(Alignment::new(vec![], 0));
//     serde_wasm_bindgen::to_value(&alignment).unwrap_or(JsValue::NULL)
// }

/**
 * @param {Vec<Vec<i32>>} score_matrix
 * @returns {Vec<Vec<i32>>} merges list containing pairs of indices to merge
 */
#[wasm_bindgen]
pub fn wasm_gusfield_mst(score_matrix: String) -> JsValue {
    let score_matrix: Vec<Vec<i32>> = serde_json::from_str(&score_matrix).unwrap();
    let merges = gusfields::gusfield_mst(&score_matrix);
    serde_wasm_bindgen::to_value(&merges).unwrap_or(JsValue::NULL)
}

/**
 * @param {Vec<Vec<i32>>} score_matrix
 * @returns {Vec<Vec<i32>>} merges list containing pairs of indices to merge
 */
#[wasm_bindgen]
pub fn wasm_kruskal_mst(score_matrix: String) -> JsValue {
    let score_matrix: Vec<Vec<i32>> = serde_json::from_str(&score_matrix).unwrap();
    let merges = gusfields::kruskal_mst(&score_matrix).unwrap();
    serde_wasm_bindgen::to_value(&merges).unwrap_or(JsValue::NULL)
}


#[wasm_bindgen]
pub fn msa_wasm(fasta: String, substitution_matrix: String, gap_cost: i32, algorithm: String) -> JsValue {
    let output: (Vec<Vec<Vec<Vec<u8>>>>, i32) = msa(&substitution_matrix, gap_cost, false, &fasta, &algorithm).unwrap();
    let steps: Vec<Vec<Vec<Vec<u8>>>> = output.0;
    let score = output.1;
    let mut js_output: String = String::from("");
    for step in steps {
        for cluster in step {
            for seq in cluster {
                js_output = js_output + std::str::from_utf8(&seq).unwrap();
            }
            // remove last &
            js_output.pop();
            js_output = js_output + "$";
        }
        // remove last $
        js_output.pop();
        js_output = js_output + "#";
    }
    // add score at the end
    js_output = js_output + score.to_string().as_str();
    return JsValue::from_str(js_output.as_str());
}



// /**
//  * @param {String} str1: string from first cluster that is being merged
//  * @param {String} str_pair2: pairwise alignment from second cluster that is being merged
//  * @param {String} str_pair1: pairwise alignment from first cluster that is being merged
//  * @returns {Vec<<i32>} list of integers representing instructions for merging (1, 2 or 3)
//  */
// #[wasm_bindgen]
// pub fn wasm_merge_instructions(str1: String, str2: String, str1_pair: String, str2_pair: String) -> JsValue {
//     // send parameters as arguments but instead of Strings they are Vec<u8>
//     let instructions: Vec<i32> = gusfields::merge_clusters(Vec::from(str1), Vec::from(str2), Vec::from(str1_pair), Vec::from(str2_pair));
//     return serde_wasm_bindgen::to_value(&instructions).unwrap_or(JsValue::NULL);
// }


// #[wasm_bindgen]
// pub fn wasm_gusfields(fasta_string: String, submatrix_string: String, gap_cost: i32, maximize: bool) -> JsValue {
//     let records = parse_fasta_string(fasta_string);
//     let sm = parse_submatrix_string(submatrix_string);
//     let alignment = gusfield_msa(&records, &sm, gap_cost, maximize).unwrap();
//     serde_wasm_bindgen::to_value(&alignment).unwrap_or(JsValue::NULL)
// }