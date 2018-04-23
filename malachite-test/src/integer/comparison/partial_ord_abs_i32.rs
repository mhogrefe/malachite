use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};
use malachite_base::num::PartialOrdAbs;
use malachite_base::num::SignificantBits;
use std::cmp::Ordering;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_cmp_abs_i32);
    register_demo!(registry, demo_i32_partial_cmp_abs_integer);
    register_bench!(registry, Large, benchmark_integer_partial_cmp_abs_i32);
    register_bench!(registry, Large, benchmark_i32_partial_cmp_abs_integer);
}

fn demo_integer_partial_cmp_abs_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        match n.partial_cmp_abs(&i).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, i),
            Ordering::Equal => println!("|{}| = |{}|", n, i),
            Ordering::Greater => println!("|{}| > |{}|", n, i),
        }
    }
}

fn demo_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        match PartialOrdAbs::partial_cmp_abs(&i, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", i, n),
            Ordering::Equal => println!("|{}| = |{}|", i, n),
            Ordering::Greater => println!("|{}| > |{}|", i, n),
        }
    }
}

fn benchmark_integer_partial_cmp_abs_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.partial_cmp_abs(&i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32.partial_cmp_abs(&Integer)",
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}
