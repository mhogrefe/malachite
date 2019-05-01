use malachite_base::chars::contiguous_range_to_char;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_contiguous_range_to_char);
    register_bench!(registry, None, benchmark_contiguous_range_to_char);
}

fn demo_contiguous_range_to_char(gm: GenerationMode, limit: usize) {
    for i in unsigneds(gm).take(limit) {
        println!(
            "contiguous_range_to_char({}) = {:?}",
            i,
            contiguous_range_to_char(i)
        );
    }
}

fn benchmark_contiguous_range_to_char(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "contiguous_range_to_char(char)",
        BenchmarkType::Single,
        unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| i as usize),
        "i",
        &mut [("malachite", &mut (|i| no_out!(contiguous_range_to_char(i))))],
    );
}
