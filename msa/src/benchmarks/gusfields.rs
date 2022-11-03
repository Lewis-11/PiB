use msa::{alignment::gusfield_msa, fasta::read_fasta_file, utils::read_submatrix_file};
use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

fn gusfield_msa_benchmkark(c: &mut Criterion) {
    let test_fasta_file = "./input/test_short.fasta";
    let test_sub_file = "./input/submat.txt";
    let gap_cost = 5;
    let records = read_fasta_file(test_fasta_file);
    let sub_matrix = read_submatrix_file(test_sub_file);
    let mut group = c.benchmark_group("Gusfield MSA alignment");

    let parameter_id= format!("test_short");
    let parameters = (
        &records,
        &sub_matrix,
        gap_cost,
        false
    );
    group.bench_with_input(
        BenchmarkId::new("Default", parameter_id),
        &parameters,
        |b, (records, subm, gc, maximize)| b.iter(|| gusfield_msa(records, subm, *gc, *maximize))
    );
    group.finish();
}
criterion_group!(benches, gusfield_msa_benchmkark);
criterion_main!(benches);