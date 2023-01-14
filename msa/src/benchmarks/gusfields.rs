use msa::{alignment::msa, fasta::read_fasta_file, utils::read_submatrix_file};
use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};

fn gusfield_msa_benchmkark(c: &mut Criterion) {

    let mut group = c.benchmark_group("Gusfield MSA alignment");
    
    let test_file_collection = [
        "input/chrUn_GL4563_100.fa",
        "input/chrUn_GL4563_200.fa",
        "input/chrUn_GL4563_300.fa",
        "input/chrUn_GL4563_400.fa",
        "input/chrUn_GL4563_500.fa",
        "input/chrUn_GL4563_600.fa"
    ];
    let test_sub_file = "./input/submat.txt";
    let gap_cost = 5;
    
    for test_file in test_file_collection {
        let records = read_fasta_file(test_file);
        let sub_matrix = read_submatrix_file(test_sub_file);

        let parameters_gusfield = (&sub_matrix,gap_cost,false,&records,String::from("gusfield"));
        group.bench_with_input(
            BenchmarkId::new("Gusfield", test_file),
            &parameters_gusfield,
            |b, (subm, gc, maximize, records, algorithm)| b.iter(|| msa(subm, *gc, *maximize, &records, algorithm))
        );

        let parameters_kruskal = (&sub_matrix,gap_cost,false,&records,String::from("kruskal"));
        group.bench_with_input(
            BenchmarkId::new("Kruskal", test_file),
            &parameters_kruskal,
            |b, (subm, gc, maximize, records, algorithm)| b.iter(|| msa(subm, *gc, *maximize, &records, algorithm))
        );
    }
    group.finish();
}
criterion_group!(benches, gusfield_msa_benchmkark);
criterion_main!(benches);