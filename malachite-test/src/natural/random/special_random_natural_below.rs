use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::{IsaacRng, SeedableRng};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string_binary};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::special_random_natural_below_old;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::positive_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_special_random_natural_below);
    register_bench!(
        registry,
        Large,
        benchmark_natural_special_random_natural_below
    );
}

fn demo_natural_special_random_natural_below(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| special_random_natural_below_old(&mut rng, &n));
        println!(
            "special_random_natural_below({}) = {}",
            n,
            to_limited_string_binary(10, &mut xs)
        );
    }
}

fn benchmark_natural_special_random_natural_below(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    run_benchmark_old(
        "special_random_natural_below(&mut Rng, &Natural)",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|ref n| no_out!(special_random_natural_below_old(&mut rng, n))),
        )],
    );
}
