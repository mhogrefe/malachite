use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::{small_positive_unsigneds, small_unsigneds};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
use rand::{IsaacRng, SeedableRng};
use rust_wheels::iterators::adaptors::{
    generate_from_function, to_limited_string_binary, to_limited_string_debug,
};
use rust_wheels::iterators::common::EXAMPLE_SEED;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_limbs_special_random_up_to_bits);
    register_ns_demo!(registry, demo_natural_special_random_natural_up_to_bits);
    register_ns_bench!(registry, Large, benchmark_limbs_special_random_up_to_bits);
    register_ns_bench!(
        registry,
        Large,
        benchmark_natural_special_random_natural_up_to_bits
    );
}

fn demo_limbs_special_random_up_to_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_positive_unsigneds(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs =
            generate_from_function(|| limbs_special_random_up_to_bits::<u32, _>(&mut rng, bits));
        println!(
            "limbs_special_random_up_to_bits({}) = {:?}",
            bits,
            to_limited_string_debug(10, &mut xs)
        );
    }
}

fn demo_natural_special_random_natural_up_to_bits(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
        let mut xs = generate_from_function(|| special_random_natural_up_to_bits(&mut rng, bits));
        println!(
            "special_random_natural_up_to_bits({}) = {}",
            bits,
            to_limited_string_binary(10, &mut xs)
        );
    }
}

fn benchmark_limbs_special_random_up_to_bits(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    m_run_benchmark(
        "limbs_special_random_up_to_bits(&mut Rng, u64)",
        BenchmarkType::Single,
        small_positive_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::checked_from(bits).unwrap()),
        "bits",
        &mut [(
            "malachite",
            &mut (|bits| no_out!(limbs_special_random_up_to_bits::<u32, _>(&mut rng, bits))),
        )],
    );
}

fn benchmark_natural_special_random_natural_up_to_bits(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    m_run_benchmark(
        "special_random_natural_up_to_bits(&mut Rng, u64)",
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::checked_from(bits).unwrap()),
        "bits",
        &mut [(
            "malachite",
            &mut (|bits| no_out!(special_random_natural_up_to_bits(&mut rng, bits))),
        )],
    );
}
