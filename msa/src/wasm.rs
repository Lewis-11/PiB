use std::collections::HashMap;
use fasta::parse_fasta_string;
use utils::parse_submatrix_string;
use alignment::gusfield_msa;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use crate::fasta::{Alignment, FastaSequence};

mod fasta;
mod utils;
mod alignment;
mod adjacency_matrix;
mod gusfields;
mod msa;

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

/**
 * @param {FastaSequence} seq1 {name: string, sequence: string}
 * @param {FastaSequence} seq1 {name: string, sequence: string}
 * @param {HashMap [u8][u8] -> i32} sub_matrix
 * @param {number} gap_cost
 * @param {boolean} maximize
 * @returns {Alignment} {sequences: [FastaSequence, FastaSequence], score: number}
 */
#[wasm_bindgen]
pub fn wasm_pairwise_alignment(seq1: FastaSequence, seq2: FastaSequence, sub_matrix: &HashMap<u8, HashMap<u8, i32>>, gap_cost: i32, maximize: bool) -> Alignment {
    return alignment::pairwise_alignment(&seq1, &seq2, sub_matrix, gap_cost, maximize).unwrap_or(Alignment::new(vec![], 0));
}

#[wasm_bindgen]
pub fn wasm_gusfields(fasta_string: String, submatrix_string: String, gap_cost: i32, maximize: bool) -> JsValue {
    let records = parse_fasta_string(fasta_string);
    let sm = parse_submatrix_string(submatrix_string);
    let alignment = gusfield_msa(&records, &sm, gap_cost, maximize).unwrap();
    serde_wasm_bindgen::to_value(&alignment).unwrap()
}