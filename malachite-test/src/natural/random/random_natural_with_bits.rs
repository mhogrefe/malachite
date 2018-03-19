use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use inputs::base::small_u64s;
use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
use rand::{IsaacRng, SeedableRng};
use rust_wheels::iterators::adaptors::{generate_from_function, to_limited_string};
use rust_wheels::iterators::common::EXAMPLE_SEED;

pub fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_natural_random_natural_with_bits);
    register_ns_bench!(registry, Large, benchmark_natural_random_natural_with_bits);
}

fn demo_natural_random_natural_with_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_u64s(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| random_natural_with_bits(&mut rng, bits));
        println!(
            "random_natural_with_bits({}) = {}",
            bits,
            to_limited_string(10, &mut xs)
        );
    }
}

fn benchmark_natural_random_natural_with_bits(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    m_run_benchmark(
        "random_natural_with_bits(&mut Rng, u64)",
        BenchmarkType::Single,
        small_u64s(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| bits as usize),
        "bits",
        &mut [
            (
                "malachite",
                &mut (|bits| no_out!(random_natural_with_bits(&mut rng, bits))),
            ),
        ],
    );
}
