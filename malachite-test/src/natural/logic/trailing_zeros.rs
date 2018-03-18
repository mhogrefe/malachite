use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

pub fn demo_natural_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

pub fn benchmark_natural_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.trailing_zeros()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.trailing_zeros())))],
    );
}
