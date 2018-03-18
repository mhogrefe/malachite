use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

pub fn demo_integer_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

pub fn benchmark_integer_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.trailing_zeros()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.trailing_zeros())))],
    );
}
