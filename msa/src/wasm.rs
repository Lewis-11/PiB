use fasta::parse_fasta_string;
use utils::map_seq_name_to_id;
use utils::parse_submatrix_string;
use alignment::gusfield_msa;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod fasta;
pub mod utils;
pub mod alignment;
pub mod adjacency_matrix;
pub mod gusfields;
pub mod kruskal;

#[wasm_bindgen]
pub fn wasm_serialize_fasta_string(fasta_string: String) -> JsValue {
    let sequences = parse_fasta_string(fasta_string);
    serde_wasm_bindgen::to_value(&sequences).unwrap()
}

#[wasm_bindgen]
pub fn wasm_serialize_submatrix(submatrix_string: String) -> JsValue {
    let submatrix_cost = parse_submatrix_string(submatrix_string);
    serde_wasm_bindgen::to_value(&submatrix_cost).unwrap()
}

#[wasm_bindgen]
pub fn wasm_gusfields(fasta_string: String, submatrix_string: String, gap_cost: i32, maximize: bool) -> JsValue {
    let records = parse_fasta_string(fasta_string);
    let sm = parse_submatrix_string(submatrix_string);
    let seq_id_map = map_seq_name_to_id(&records);
    let alignment = gusfield_msa(&records, &sm, &seq_id_map, gap_cost, maximize).unwrap();
    serde_wasm_bindgen::to_value(&alignment).unwrap()
}