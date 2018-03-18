use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{naturals, nrm_naturals};
use malachite_base::num::SignificantBits;

pub fn demo_natural_neg(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

pub fn demo_natural_neg_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

pub fn benchmark_natural_neg_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "-Natural",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(-n))),
            ("num", &mut (|(n, _, _)| no_out!(-n))),
            ("rug", &mut (|(_, n, _)| no_out!(-n))),
        ],
    );
}

pub fn benchmark_natural_neg_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "-Natural",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("-Natural", &mut (|n| no_out!(-n))),
            ("-&Natural", &mut (|n| no_out!(-&n))),
        ],
    );
}
