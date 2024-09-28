use std::hint::black_box;

use rand_pcg::Pcg64Mcg;
use rand_core::SeedableRng;
use lipsum::lipsum_with_rng;

use pig_latin::translate;

fn main(){ 
    let rng = Pcg64Mcg::seed_from_u64(0);
    let test_input = lipsum_with_rng(rng, 1_000_000);
    translate(black_box(&test_input));
}