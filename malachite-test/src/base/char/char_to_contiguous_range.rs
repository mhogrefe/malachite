use common::{m_run_benchmark, BenchmarkType, NoSpecialGenerationMode};
use inputs::base::chars;
use malachite_base::chars::char_to_contiguous_range;

pub fn demo_char_to_contiguous_range(gm: NoSpecialGenerationMode, limit: usize) {
    for c in chars(gm).take(limit) {
        println!(
            "char_to_contiguous_range({:?}) = {}",
            c,
            char_to_contiguous_range(c)
        );
    }
}

pub fn benchmark_char_to_contiguous_range(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "char_to_contiguous_range(char)",
        BenchmarkType::Single,
        chars(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| char_to_contiguous_range(c) as usize),
        "char_to_contiguous_range(char)",
        &[
            ("malachite", &mut (|c| no_out!(char_to_contiguous_range(c)))),
        ],
    );
}
