use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::integers;

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
    run_benchmark(
        "Integer.even()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.even())))],
    );
}

fn benchmark_integer_odd(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.odd()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.odd())))],
    );
}
