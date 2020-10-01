use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::random::random_natural_below::random_natural_below;
use rand::{IsaacRng, SeedableRng};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::positive_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_random_natural_below);
    register_bench!(registry, Large, benchmark_natural_random_natural_below);
}

fn demo_natural_random_natural_below(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| random_natural_below(&mut rng, &n));
        println!(
            "random_natural_below({}) = {}",
            n,
            to_limited_string(10, &mut xs)
        );
    }
}

fn benchmark_natural_random_natural_below(gm: GenerationMode, limit: usize, file_name: &str) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    run_benchmark_old(
        "random_natural_below(&mut Rng, &Natural)",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|ref n| no_out!(random_natural_below(&mut rng, n))),
        )],
    );
}
