use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::unsigneds;
use malachite_base::chars::contiguous_range_to_char;

pub fn demo_contiguous_range_to_char(gm: GenerationMode, limit: usize) {
    for i in unsigneds(gm).take(limit) {
        println!(
            "contiguous_range_to_char({}) = {:?}",
            i,
            contiguous_range_to_char(i)
        );
    }
}

pub fn benchmark_contiguous_range_to_char(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "contiguous_range_to_char(char)",
        BenchmarkType::Single,
        unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| i as usize),
        "i",
        &[
            ("malachite", &mut (|i| no_out!(contiguous_range_to_char(i)))),
        ],
    );
}
