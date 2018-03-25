use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u64_checked_from_natural);
    register_demo!(registry, demo_u64_wrapping_from_natural);
    register_bench!(registry, Large, benchmark_u64_checked_from_natural);
    register_bench!(registry, Large, benchmark_u64_wrapping_from_natural);
}

fn demo_u64_checked_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("u64::checked_from(&{}) = {:?}", n, u64::checked_from(&n));
    }
}

fn demo_u64_wrapping_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("u64::wrapping_from(&{}) = {}", n, u64::wrapping_from(&n));
    }
}

fn benchmark_u64_checked_from_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u64::checked_from(&Natural)",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(u64::checked_from(&n))))],
    );
}

fn benchmark_u64_wrapping_from_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u64::wrapping_from(&Natural)",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(u64::wrapping_from(&n))))],
    );
}
