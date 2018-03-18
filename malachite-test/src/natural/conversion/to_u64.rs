use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

pub fn demo_natural_to_u64(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64());
    }
}

pub fn demo_natural_to_u64_wrapping(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64_wrapping());
    }
}

pub fn benchmark_natural_to_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.to_u64()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.to_u64())))],
    );
}

pub fn benchmark_natural_to_u64_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.to_u64_wrapping()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.to_u64_wrapping())))],
    );
}
