use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_is_even);
    register_demo!(registry, demo_natural_is_odd);
    register_bench!(registry, Large, benchmark_natural_is_even);
    register_bench!(registry, Large, benchmark_natural_is_odd);
}

fn demo_natural_is_even(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        if n.is_even() {
            println!("{} is even", n);
        } else {
            println!("{} is not even", n);
        }
    }
}

fn demo_natural_is_odd(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        if n.is_odd() {
            println!("{} is odd", n);
        } else {
            println!("{} is not odd", n);
        }
    }
}

fn benchmark_natural_is_even(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.is_even()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.is_even())))],
    );
}

fn benchmark_natural_is_odd(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.is_odd()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.is_odd())))],
    );
}
