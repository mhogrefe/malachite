use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::SignificantBits;
use malachite_nz::natural::Natural;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_checked_from_integer);
    register_demo!(registry, demo_natural_checked_from_ref_integer);
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_from_integer_evaluation_strategy
    );
}

fn demo_natural_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::checked_from({}) = {:?}",
            n_clone,
            Natural::checked_from(n)
        );
    }
}

fn demo_natural_checked_from_ref_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "Natural::checked_from(&{}) = {:?}",
            n,
            Natural::checked_from(&n)
        );
    }
}

fn benchmark_natural_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural::checked_from(Integer)",
                &mut (|n| no_out!(Natural::checked_from(n))),
            ),
            (
                "Natural::checked_from(&Integer)",
                &mut (|n| no_out!(Natural::checked_from(&n))),
            ),
        ],
    );
}
