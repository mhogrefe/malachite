use malachite_base::chars::char_to_contiguous_range;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::chars;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_char_to_contiguous_range);
    register_ns_bench!(registry, None, benchmark_char_to_contiguous_range);
}

fn demo_char_to_contiguous_range(gm: NoSpecialGenerationMode, limit: usize) {
    for c in chars(gm).take(limit) {
        println!(
            "char_to_contiguous_range({:?}) = {}",
            c,
            char_to_contiguous_range(c)
        );
    }
}

fn benchmark_char_to_contiguous_range(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "char_to_contiguous_range(char)",
        BenchmarkType::Single,
        chars(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|c| no_out!(char_to_contiguous_range(c))))],
    );
}
