use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::integer::Integer;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_integer_power_of_two);
    register_ns_bench!(registry, Large, benchmark_integer_power_of_two);
}

fn demo_integer_power_of_two(gm: NoSpecialGenerationMode, limit: usize) {
    for pow in small_unsigneds(gm).take(limit) {
        println!("2^{} = {}", pow, Integer::power_of_two(pow));
    }
}

fn benchmark_integer_power_of_two(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        &format!("Integer.power_of_two(u64)"),
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&pow| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|pow| no_out!(Integer::power_of_two(pow))),
        )],
    );
}
