use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::random::random_natural_up_to_bits::{
    limbs_random_up_to_bits, random_natural_up_to_bits,
};
use rand::{IsaacRng, SeedableRng};
use rust_wheels::iterators::adaptors::{
    generate_from_function, to_limited_string, to_limited_string_debug,
};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::{small_positive_unsigneds, small_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_limbs_random_up_to_bits);
    register_ns_demo!(registry, demo_natural_random_natural_up_to_bits);
    register_ns_bench!(registry, Large, benchmark_limbs_random_up_to_bits);
    register_ns_bench!(registry, Large, benchmark_natural_random_natural_up_to_bits);
}

fn demo_limbs_random_up_to_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_positive_unsigneds(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| limbs_random_up_to_bits::<u32, _>(&mut rng, bits));
        println!(
            "limbs_random_up_to_bits({}) = {:?}",
            bits,
            to_limited_string_debug(10, &mut xs)
        );
    }
}

fn demo_natural_random_natural_up_to_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| random_natural_up_to_bits(&mut rng, bits));
        println!(
            "random_natural_up_to_bits({}) = {}",
            bits,
            to_limited_string(10, &mut xs)
        );
    }
}

fn benchmark_limbs_random_up_to_bits(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    m_run_benchmark(
        "limbs_random_up_to_bits(&mut Rng, u64)",
        BenchmarkType::Single,
        small_positive_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [(
            "malachite",
            &mut (|bits| no_out!(limbs_random_up_to_bits::<u32, _>(&mut rng, bits))),
        )],
    );
}

fn benchmark_natural_random_natural_up_to_bits(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    m_run_benchmark(
        "random_natural_up_to_bits(&mut Rng, u64)",
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [(
            "malachite",
            &mut (|bits| no_out!(random_natural_up_to_bits(&mut rng, bits))),
        )],
    );
}
