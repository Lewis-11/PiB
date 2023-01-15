use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use crate::alignment::msa;

pub mod fasta;
pub mod utils;
pub mod alignment;
pub mod adjacency_matrix;
pub mod gusfields;

/**
 * @param {Vec<Vec<i32>>} score_matrix
 * @returns {Vec<Vec<i32>>} merges list containing pairs of indices to merge
 */
#[wasm_bindgen]
pub fn wasm_gusfield_mst(score_matrix: String, maximize: bool) -> JsValue {
    let score_matrix: Vec<Vec<i32>> = serde_json::from_str(&score_matrix).unwrap();
    let merges = gusfields::gusfield_mst(&score_matrix, maximize);
    serde_wasm_bindgen::to_value(&merges).unwrap_or(JsValue::NULL)
}

/**
 * @param {Vec<Vec<i32>>} score_matrix
 * @returns {Vec<Vec<i32>>} merges list containing pairs of indices to merge
 */
#[wasm_bindgen]
pub fn wasm_kruskal_mst(score_matrix: String, maximize: bool) -> JsValue {
    let score_matrix: Vec<Vec<i32>> = serde_json::from_str(&score_matrix).unwrap();
    let merges = gusfields::kruskal_mst(&score_matrix, maximize).unwrap();
    serde_wasm_bindgen::to_value(&merges).unwrap_or(JsValue::NULL)
}


#[wasm_bindgen]
pub fn msa_wasm(fasta: String, substitution_matrix: String, gap_cost: i32, maximize: bool, algorithm: String) -> JsValue {
    let output: (Vec<Vec<Vec<Vec<u8>>>>, i32) = msa(&substitution_matrix, gap_cost, maximize, &fasta, &algorithm).unwrap();
    let steps: Vec<Vec<Vec<Vec<u8>>>> = output.0;
    let score = output.1;
    let mut js_output: String = String::from("");
    for step in steps {
        for cluster in step {
            for seq in cluster {
                js_output = js_output + std::str::from_utf8(&seq).unwrap() + "&";
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