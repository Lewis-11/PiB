use msa::fasta::parse_fasta_string;
use msa::fasta::read_fasta_file;
use msa::utils::parse_submatrix_string;
use msa::utils::read_submatrix_file;
use msa::alignment::iterative_pairwise_alignment_cost;
use msa::alignment::iterative_backtracking;
use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

fn pairwise_alignment_benchmark(c: &mut Criterion) {
    let gap_cost = 5;
    let fasta_string = read_fasta_file("./input/test_short.fasta");
    let records = parse_fasta_string(&fasta_string);
    let submatrix_string = read_submatrix_file("./input/submat.txt");
    let sub_matrix = parse_submatrix_string(&submatrix_string);
    let mut group = c.benchmark_group("Iterative alignment");
    
    for i in 0..records.len() {
        for j in i+1..records.len() {
            let parameter_id
                = format!("{}-{}", records[i].name, records[j].name);
            let parameters = (
                &records[i],
                &records[j],
                &sub_matrix,
                gap_cost,
                false
            );
            group.bench_with_input(
                BenchmarkId::new("Default", parameter_id),
                &parameters,
                |b, (s1, s2, subm, gc, maximize)| b.iter(|| iterative_pairwise_alignment_cost(s1, s2, subm, *gc, *maximize))
            );
        }
    }
    group.finish();
}

fn pairwise_backtracking_benchmark(c: &mut Criterion) {
    let gap_cost = 5;
    let fasta_string = read_fasta_file("./input/test_short.fasta");
    let records = parse_fasta_string(&fasta_string);
    let submatrix_string = read_submatrix_file("./input/submat.txt");
    let sub_matrix = parse_submatrix_string(&submatrix_string);
    let mut group = c.benchmark_group("Iterative backtracking");
    
    for i in 0..records.len() {
        for j in i+1..records.len() {
            let parameter_id
                = format!("{}-{}", records[i].name, records[j].name);
            let score_matrix = iterative_pairwise_alignment_cost(
                &records[i],
                &records[j],
                &sub_matrix,
                gap_cost,
                false
            ).unwrap();
            let parameters = (
                &score_matrix,
                &records[i],
                &records[j],
                &sub_matrix,
                gap_cost
            );
            group.bench_with_input(
                BenchmarkId::new("Default", parameter_id),
                &parameters,
                |b, (scorem, s1, s2, subm, gc)| b.iter(|| iterative_backtracking(scorem, s1, s2, subm, *gc))
            );
        }
    }
    group.finish();
}

criterion_group!(benches, pairwise_alignment_benchmark, pairwise_backtracking_benchmark);
criterion_main!(benches);