use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i64_checked_from_integer);
    register_demo!(registry, demo_i64_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_i64_checked_from_integer);
    register_bench!(registry, Large, benchmark_i64_wrapping_from_integer);
}

fn demo_i64_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("i64::checked_from(&{}) = {:?}", n, i64::checked_from(&n));
    }
}

fn demo_i64_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("i64::wrapping_from(&{}) = {}", n, i64::wrapping_from(&n));
    }
}

fn benchmark_i64_checked_from_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i64::checked_from(&Integer)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(i64::checked_from(&n))))],
    );
}

fn benchmark_i64_wrapping_from_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i64::wrapping_from(&Integer)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(i64::wrapping_from(&n))))],
    );
}
