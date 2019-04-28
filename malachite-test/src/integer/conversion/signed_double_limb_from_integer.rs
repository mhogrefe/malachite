use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::traits::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_i64_checked_from_integer);
    register_demo!(registry, demo_i64_checked_from_integer_ref);
    register_demo!(registry, demo_i64_wrapping_from_integer);
    register_demo!(registry, demo_i64_wrapping_from_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_i64_checked_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_wrapping_from_integer_evaluation_strategy
    );
}

fn demo_i64_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "i64::checked_from({}) = {:?}",
            n_clone,
            i64::checked_from(n)
        );
    }
}

fn demo_i64_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("i64::checked_from(&{}) = {:?}", n, i64::checked_from(&n));
    }
}

fn demo_i64_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "i64::wrapping_from({}) = {}",
            n_clone,
            i64::wrapping_from(n)
        );
    }
}

fn demo_i64_wrapping_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("i64::wrapping_from(&{}) = {}", n, i64::wrapping_from(&n));
    }
}

fn benchmark_i64_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i64::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i64::checked_from(Integer)",
                &mut (|n| no_out!(i64::checked_from(n))),
            ),
            (
                "i64::checked_from(&Integer)",
                &mut (|n| no_out!(i64::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_i64_wrapping_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i64::wrapping_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i64::wrapping_from(Integer)",
                &mut (|n| no_out!(i64::wrapping_from(n))),
            ),
            (
                "i64::wrapping_from(&Integer)",
                &mut (|n| no_out!(i64::wrapping_from(&n))),
            ),
        ],
    );
}
