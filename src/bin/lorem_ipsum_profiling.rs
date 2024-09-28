use std::hint::black_box;

use rand_pcg::Pcg64Mcg;
use rand_core::SeedableRng;
use lipsum::lipsum_with_rng;

use pig_latin::translate;

fn main(){
    let mut total_in_bytes = 0; 
    let mut total_out_bytes = 0; 
    for i in 0..10 {
        let rng = Pcg64Mcg::seed_from_u64(i);
        let test_input = lipsum_with_rng(rng, 1_000_000);
        total_in_bytes += test_input.len();
        println!("passing {} bytes to translate ...", test_input.len());
        let result = translate(black_box(&test_input));
        println!("... translated into {} bytes", result.len());
        total_out_bytes += result.len();
    }
    println!("translated {total_in_bytes} UTF8 bytes into {total_out_bytes} UTF8 bytes");
}