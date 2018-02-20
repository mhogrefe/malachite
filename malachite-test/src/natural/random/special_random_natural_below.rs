use common::GenerationMode;
use inputs::natural::positive_naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::random::special_random_natural_below::special_random_natural_below;
use rand::{IsaacRng, SeedableRng};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string_binary};
use rust_wheels::iterators::common::EXAMPLE_SEED;

pub fn demo_natural_special_random_natural_below(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| special_random_natural_below(&mut rng, &n));
        println!(
            "special_random_natural_below({}) = {}",
            n,
            to_limited_string_binary(10, &mut xs)
        );
    }
}

pub fn benchmark_natural_special_random_natural_below(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural::special_random_natural_below(&mut Rng, &Natural)",
        gm.name()
    );
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    benchmark_1(BenchmarkOptions1 {
        xs: positive_naturals(gm),
        function_f: &mut (|ref n| special_random_natural_below(&mut rng, n)),
        x_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural::special\\\\_random\\\\_natural\\\\_below(\\\\&mut Rng, &Natural)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
