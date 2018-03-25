use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u64_checked_from_integer);
    register_demo!(registry, demo_u64_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_u64_checked_from_integer);
    register_bench!(registry, Large, benchmark_u64_wrapping_from_integer);
}

fn demo_u64_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("u64::checked_from(&{}) = {:?}", n, u64::checked_from(&n));
    }
}

fn demo_u64_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("u64::wrapping_from(&{}) = {}", n, u64::wrapping_from(&n));
    }
}

fn benchmark_u64_checked_from_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u64::checked_from(&Integer)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(u64::checked_from(&n))))],
    );
}

fn benchmark_u64_wrapping_from_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u64::wrapping_from(&Integer)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(u64::wrapping_from(&n))))],
    );
}
