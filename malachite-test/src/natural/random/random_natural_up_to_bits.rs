use common::GenerationMode;
use inputs::base::small_u64s;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::random::random_natural_up_to_bits::random_natural_up_to_bits;
use rand::{IsaacRng, SeedableRng};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string};
use rust_wheels::iterators::common::EXAMPLE_SEED;

pub fn demo_natural_random_natural_up_to_bits(gm: GenerationMode, limit: usize) {
    for bits in small_u64s(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| random_natural_up_to_bits(&mut rng, bits));
        println!(
            "random_natural_up_to_bits({}) = {}",
            bits,
            to_limited_string(10, &mut xs)
        );
    }
}

pub fn benchmark_natural_random_natural_up_to_bits(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural::random_natural_up_to_bits(&mut Rng, u64)",
        gm.name()
    );
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    benchmark_1(BenchmarkOptions1 {
        xs: small_u64s(gm),
        function_f: &mut (|bits| random_natural_up_to_bits(&mut rng, bits)),
        x_cons: &(|&bits| bits),
        x_param: &(|&bits| bits.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural::random\\\\_natural\\\\_up\\\\_to\\\\_bits(\\\\&mut Rng, u64)",
        x_axis_label: "bits.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
