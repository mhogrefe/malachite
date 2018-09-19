use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, rm_integers};
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u32_checked_from_integer);
    register_demo!(registry, demo_u32_checked_from_integer_ref);
    register_demo!(registry, demo_u32_wrapping_from_integer);
    register_demo!(registry, demo_u32_wrapping_from_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_u32_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_checked_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_wrapping_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_wrapping_from_integer_evaluation_strategy
    );
}

fn demo_u32_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "u32::checked_from({}) = {:?}",
            n_clone,
            u32::checked_from(n)
        );
    }
}

fn demo_u32_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("u32::checked_from(&{}) = {:?}", n, u32::checked_from(&n));
    }
}

fn demo_u32_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "u32::wrapping_from({}) = {}",
            n_clone,
            u32::wrapping_from(n)
        );
    }
}

fn demo_u32_wrapping_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("u32::wrapping_from(&{}) = {}", n, u32::wrapping_from(&n));
    }
}

fn benchmark_u32_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(u32::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32()))),
        ],
    );
}

fn benchmark_u32_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32::checked_from(Integer)",
                &mut (|n| no_out!(u32::checked_from(n))),
            ),
            (
                "u32::checked_from(&Integer)",
                &mut (|n| no_out!(u32::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_u32_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(u32::wrapping_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32_wrapping()))),
        ],
    );
}

fn benchmark_u32_wrapping_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32::wrapping_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32::wrapping_from(Integer)",
                &mut (|n| no_out!(u32::wrapping_from(n))),
            ),
            (
                "u32::wrapping_from(&Integer)",
                &mut (|n| no_out!(u32::wrapping_from(&n))),
            ),
        ],
    );
}
