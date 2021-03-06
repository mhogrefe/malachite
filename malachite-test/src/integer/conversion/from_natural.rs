use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::integer::Integer;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::naturals;

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
    run_benchmark_old(
        "Integer::from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
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
