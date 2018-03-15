use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

pub fn demo_integer_to_u64(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64());
    }
}

pub fn demo_integer_to_u64_wrapping(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_u64_wrapping({}) = {:?}", n, n.to_u64_wrapping());
    }
}

pub fn benchmark_integer_to_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_u64()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|n| no_out!(n.to_u64())))],
    );
}

pub fn benchmark_integer_to_u64_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_u64_wrapping()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|n| no_out!(n.to_u64_wrapping())))],
    );
}
