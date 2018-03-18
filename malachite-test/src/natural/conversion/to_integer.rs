use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

pub fn demo_natural_into_integer(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!("into_integer({}) = {}", n_clone, n.into_integer());
    }
}

pub fn demo_natural_to_integer(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_integer(&{}) = {}", n, n.to_integer());
    }
}

pub fn benchmark_natural_to_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.to_integer()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural.to_integer()", &mut (|n| no_out!(n.to_integer()))),
            (
                "Natural.into_integer()",
                &mut (|n| no_out!(n.into_integer())),
            ),
        ],
    );
}
