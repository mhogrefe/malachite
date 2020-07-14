use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::Natural;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_natural_power_of_two);
    register_ns_bench!(registry, Large, benchmark_natural_power_of_two);
}

fn demo_natural_power_of_two(gm: NoSpecialGenerationMode, limit: usize) {
    for pow in small_unsigneds(gm).take(limit) {
        println!("2^{} = {}", pow, Natural::power_of_two(pow));
    }
}

fn benchmark_natural_power_of_two(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        &format!("Natural.power_of_two(u64)"),
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&pow| usize::exact_from(pow)),
        "pow",
        &mut [(
            "malachite",
            &mut (|pow| no_out!(Natural::power_of_two(pow))),
        )],
    );
}
