use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

pub fn demo_integer_to_i64(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_i64({}) = {:?}", n, n.to_i64());
    }
}

pub fn demo_integer_to_i64_wrapping(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_i64_wrapping({}) = {:?}", n, n.to_i64_wrapping());
    }
}

pub fn benchmark_integer_to_i64(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_i64()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|n| no_out!(n.to_i64())))],
    );
}

pub fn benchmark_integer_to_i64_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_i64_wrapping()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|n| no_out!(n.to_i64_wrapping())))],
    );
}
