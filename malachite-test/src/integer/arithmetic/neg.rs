use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, nrm_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_neg_assign);
    register_demo!(registry, demo_integer_neg);
    register_demo!(registry, demo_integer_neg_ref);
    register_bench!(registry, Large, benchmark_integer_neg_assign);
    register_bench!(registry, Large, benchmark_integer_neg_library_comparison);
    register_bench!(registry, Large, benchmark_integer_neg_evaluation_strategy);
}

fn demo_integer_neg_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {}; n.neg_assign(); n = {}", n_old, n);
    }
}

fn demo_integer_neg(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

fn demo_integer_neg_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

fn benchmark_integer_neg_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.neg_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.neg_assign()))],
    );
}

fn benchmark_integer_neg_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "-Integer",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(-n))),
            ("num", &mut (|(n, _, _)| no_out!(-n))),
            ("rug", &mut (|(_, n, _)| no_out!(-n))),
        ],
    );
}

fn benchmark_integer_neg_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "-Integer",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("-Integer", &mut (|n| no_out!(-n))),
            ("-&Integer", &mut (|n| no_out!(-&n))),
        ],
    );
}
