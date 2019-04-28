use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u64_checked_from_integer);
    register_demo!(registry, demo_u64_checked_from_integer_ref);
    register_demo!(registry, demo_u64_wrapping_from_integer);
    register_demo!(registry, demo_u64_wrapping_from_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_u64_checked_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_wrapping_from_integer_evaluation_strategy
    );
}

fn demo_u64_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "u64::checked_from({}) = {:?}",
            n_clone,
            u64::checked_from(n)
        );
    }
}

fn demo_u64_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("u64::checked_from(&{}) = {:?}", n, u64::checked_from(&n));
    }
}

fn demo_u64_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "u64::wrapping_from({}) = {}",
            n_clone,
            u64::wrapping_from(n)
        );
    }
}

fn demo_u64_wrapping_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("u64::wrapping_from(&{}) = {}", n, u64::wrapping_from(&n));
    }
}

fn benchmark_u64_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u64::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u64::checked_from(Integer)",
                &mut (|n| no_out!(u64::checked_from(n))),
            ),
            (
                "u64::checked_from(&Integer)",
                &mut (|n| no_out!(u64::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_u64_wrapping_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u64::wrapping_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u64::wrapping_from(Integer)",
                &mut (|n| no_out!(u64::wrapping_from(n))),
            ),
            (
                "u64::wrapping_from(&Integer)",
                &mut (|n| no_out!(u64::wrapping_from(&n))),
            ),
        ],
    );
}
