use std::hint::black_box;

use lipsum::lipsum_with_rng;
use rand_core::SeedableRng;
use rand_pcg::Pcg64Mcg;

use pig_latin::translate;

/// # Translate long lorem ipsum for profiling
///
/// Generate 4 million words of pseudo-random lorem-ipsum-ish input text,
/// and translate it into pig latin. This is a useful binary for profiling
/// the core functionality of the [`pig_latin`] crate.
///
/// For micro-benchmarking, see benches/translation_benchmark.rs instead.
fn main() {
    let mut total_in_bytes = 0;
    let mut total_out_bytes = 0;
    for i in 0..20 {
        let rng = Pcg64Mcg::seed_from_u64(i);
        let test_input = lipsum_with_rng(rng, 4_000_000);
        total_in_bytes += test_input.len();
        println!("passing {} bytes to translate ...", test_input.len());
        let result = translate(black_box(&test_input));
        println!("... translated into {} bytes", result.len());
        total_out_bytes += result.len();
    }
    println!("translated {total_in_bytes} UTF8 bytes into {total_out_bytes} UTF8 bytes");
}
