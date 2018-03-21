use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_into_natural);
    register_demo!(registry, demo_integer_to_natural);
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_natural_evaluation_strategy
    );
}

fn demo_integer_into_natural(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!("into_natural({}) = {:?}", n_clone, n.into_natural());
    }
}

fn demo_integer_to_natural(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_natural(&{}) = {:?}", n, n.to_natural());
    }
}

fn benchmark_integer_to_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_natural()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer.to_natural()", &mut (|n| no_out!(n.to_natural()))),
            (
                "Integer.into_natural()",
                &mut (|n| no_out!(n.into_natural())),
            ),
        ],
    );
}
