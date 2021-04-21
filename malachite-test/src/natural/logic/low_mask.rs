use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::logic::low_mask::limbs_low_mask;
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_limbs_low_mask);
    register_ns_demo!(registry, demo_natural_low_mask);
    register_ns_bench!(registry, Small, benchmark_limbs_low_mask);
    register_ns_bench!(registry, Large, benchmark_natural_low_mask_algorithms);
}

fn demo_limbs_low_mask(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        println!("limbs_low_mask({}) = {:?}", bits, limbs_low_mask(bits));
    }
}

fn demo_natural_low_mask(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        println!("Natural::low_mask({}) = {}", bits, Natural::low_mask(bits));
    }
}

fn benchmark_limbs_low_mask(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        &format!("limbs_low_mask(u64)"),
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [("Malachite", &mut (|bits| no_out!(limbs_low_mask(bits))))],
    );
}

fn benchmark_natural_low_mask_algorithms(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("Natural.low_mask(u64)"),
        BenchmarkType::Algorithms,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [
            (
                "Natural.low_mask(u64)",
                &mut (|bits| no_out!(Natural::low_mask(bits))),
            ),
            (
                "Natural.power_of_2(u64) - 1",
                &mut (|bits| no_out!(Natural::power_of_2(bits) - Natural::ONE)),
            ),
        ],
    );
}
