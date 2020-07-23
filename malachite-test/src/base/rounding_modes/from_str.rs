use std::str::FromStr;

use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{strings, strings_var_1};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_rounding_mode_from_str);
    register_ns_demo!(registry, demo_rounding_mode_from_str_targeted);
    register_ns_bench!(registry, Large, benchmark_rounding_mode_from_str);
}

fn demo_rounding_mode_from_str(gm: NoSpecialGenerationMode, limit: usize) {
    for s in strings(gm).take(limit) {
        println!(
            "RoundingMode::from_str({:?}) = {:?}",
            s,
            RoundingMode::from_str(&s)
        );
    }
}

fn demo_rounding_mode_from_str_targeted(gm: NoSpecialGenerationMode, limit: usize) {
    for s in strings_var_1(gm).take(limit) {
        println!(
            "RoundingMode::from_str({:?}) = {:?}",
            s,
            RoundingMode::from_str(&s)
        );
    }
}

fn benchmark_rounding_mode_from_str(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "from_str(&str)",
        BenchmarkType::Single,
        strings(gm),
        gm.name(),
        limit,
        file_name,
        &(|s| s.len()),
        "s.len()",
        &mut [("malachite", &mut (|s| no_out!(RoundingMode::from_str(&s))))],
    );
}
