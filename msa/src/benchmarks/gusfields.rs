use msa::{alignment::gusfield_msa, fasta::read_fasta_file, utils::{read_submatrix_file, map_seq_name_to_id}};
use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

fn gusfield_msa_benchmkark(c: &mut Criterion) {
    let test_fasta_file = "./input/test_short.fasta";
    let test_sub_file = "./input/submat.txt";
    let gap_cost = 5;
    let records = read_fasta_file(test_fasta_file);
    let sub_matrix = read_submatrix_file(test_sub_file);
    let seq_id_map = map_seq_name_to_id(&records);
    let mut group = c.benchmark_group("Gusfield MSA alignment");

    let parameter_id= format!("test_short");
    let parameters = (
        &records,
        &sub_matrix,
        &seq_id_map,
        gap_cost,
        false
    );
    group.bench_with_input(
        BenchmarkId::new("Default", parameter_id),
        &parameters,
        |b, (records, subm, seq_id_map, gc, maximize)| b.iter(|| gusfield_msa(records, subm, seq_id_map, *gc, *maximize))
    );
    group.finish();
}
criterion_group!(benches, gusfield_msa_benchmkark);
criterion_main!(benches);