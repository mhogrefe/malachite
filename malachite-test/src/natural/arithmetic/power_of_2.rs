use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_natural_power_of_2);
    register_ns_bench!(registry, Large, benchmark_natural_power_of_2);
}

fn demo_natural_power_of_2(gm: NoSpecialGenerationMode, limit: usize) {
    for pow in small_unsigneds(gm).take(limit) {
        println!("2^{} = {}", pow, Natural::power_of_2(pow));
    }
}

fn benchmark_natural_power_of_2(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        &format!("Natural.power_of_2(u64)"),
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&pow| usize::exact_from(pow)),
        "pow",
        &mut [("Malachite", &mut (|pow| no_out!(Natural::power_of_2(pow))))],
    );
}
