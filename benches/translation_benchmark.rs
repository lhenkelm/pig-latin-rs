use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use rand_pcg::Pcg64Mcg;
use rand_core::SeedableRng;
use lipsum::lipsum_with_rng;

use pig_latin::translate;

fn lorem_ipsum(bencher: &mut Criterion){
    for seed in [0u64, 1, 2, 42, 69, 123456789]{       
        let mut group = bencher.benchmark_group(format!("lorem_ipsum_seed{seed}"));
        for n_words in [10, 25, 50, 100, 250, 500, 1_000, 2_500, 5_000, 25_000, 50_000] {
            let rng = Pcg64Mcg::seed_from_u64(seed);
            let test_input = lipsum_with_rng(rng, n_words);
            let n_bytes = test_input.len();
            group.throughput(Throughput::Bytes(n_bytes as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("n_words:{n_words}-seed:{seed}")), 
                &n_bytes,
                |b, &_ | b.iter_with_large_drop( || translate(black_box(&test_input)))
            );
        }
        group.finish();
    }
}

criterion_group!(benches, lorem_ipsum);
criterion_main!(benches);

