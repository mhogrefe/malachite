use common::NoSpecialGenerationMode;
use inputs::base::small_u64s;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use rand::{IsaacRng, SeedableRng};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string_binary};
use rust_wheels::iterators::common::EXAMPLE_SEED;

pub fn demo_natural_special_random_natural_with_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_u64s(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| special_random_natural_with_bits(&mut rng, bits));
        println!(
            "special_random_natural_with_bits({}) = {}",
            bits,
            to_limited_string_binary(10, &mut xs)
        );
    }
}

pub fn benchmark_natural_special_random_natural_with_bits(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural::special_random_natural_with_bits(&mut Rng, u64)",
        gm.name()
    );
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    benchmark_1(BenchmarkOptions1 {
        xs: small_u64s(gm),
        function_f: &mut (|bits| special_random_natural_with_bits(&mut rng, bits)),
        x_cons: &(|&bits| bits),
        x_param: &(|&bits| bits.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural::special\\\\_random\\\\_natural\\\\_with\\\\_bits(\\\\&mut Rng, u64)",
        x_axis_label: "bits.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
