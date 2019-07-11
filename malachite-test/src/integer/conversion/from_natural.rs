use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_natural);
    register_demo!(registry, demo_integer_from_ref_natural);
    register_bench!(
        registry,
        Large,
        benchmark_integer_from_natural_evaluation_strategy
    );
}

fn demo_integer_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!("Integer::from({}) = {}", n_clone, Integer::from(n));
    }
}

fn demo_integer_from_ref_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("Integer::from(&{}) = {}", n, Integer::from(&n));
    }
}

fn benchmark_integer_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer::from(Natural)",
                &mut (|n| no_out!(Integer::from(n))),
            ),
            (
                "Integer::from(&Natural)",
                &mut (|n| no_out!(Integer::from(&n))),
            ),
        ],
    );
}
