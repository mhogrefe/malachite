use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::num::{Parity, SignificantBits};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_even);
    register_demo!(registry, demo_integer_odd);
    register_bench!(registry, Large, benchmark_integer_even);
    register_bench!(registry, Large, benchmark_integer_odd);
}

fn demo_integer_even(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        if n.even() {
            println!("{} is even", n);
        } else {
            println!("{} is not even", n);
        }
    }
}

fn demo_integer_odd(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        if n.odd() {
            println!("{} is odd", n);
        } else {
            println!("{} is not odd", n);
        }
    }
}

fn benchmark_integer_even(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.even()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.even())))],
    );
}

fn benchmark_integer_odd(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.odd()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.odd())))],
    );
}
