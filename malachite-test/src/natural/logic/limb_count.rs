use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_limb_count);
    register_bench!(registry, Large, benchmark_natural_limb_count);
}

fn demo_natural_limb_count(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

fn benchmark_natural_limb_count(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.limb_count()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|n| no_out!(n.limb_count())))],
    );
}
