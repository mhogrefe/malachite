use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use inputs::base::chars;
use malachite_base::chars::char_to_contiguous_range;

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
    m_run_benchmark(
        "char_to_contiguous_range(char)",
        BenchmarkType::Single,
        chars(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| char_to_contiguous_range(c) as usize),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|c| no_out!(char_to_contiguous_range(c))))],
    );
}
