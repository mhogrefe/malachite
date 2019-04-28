use malachite_base::num::traits::SignificantBits;
use num::{BigInt, BigUint};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{naturals, nrm_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_neg);
    register_demo!(registry, demo_natural_neg_ref);
    register_bench!(registry, Large, benchmark_natural_neg_library_comparison);
    register_bench!(registry, Large, benchmark_natural_neg_evaluation_strategy);
}

fn demo_natural_neg(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

fn demo_natural_neg_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

pub fn neg_num(u: BigUint) -> BigInt {
    -BigInt::from(u)
}

fn benchmark_natural_neg_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
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
            ("num", &mut (|(n, _, _)| no_out!(neg_num(n)))),
            ("rug", &mut (|(_, n, _)| no_out!(-n))),
        ],
    );
}

fn benchmark_natural_neg_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
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
