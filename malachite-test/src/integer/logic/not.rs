use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, rm_integers};
use malachite_base::num::NotAssign;
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_not_assign);
    register_demo!(registry, demo_integer_not);
    register_demo!(registry, demo_integer_not_ref);
    register_bench!(registry, Large, benchmark_integer_not_assign);
    register_bench!(registry, Large, benchmark_integer_not_library_comparison);
    register_bench!(registry, Large, benchmark_integer_not_evaluation_strategy);
}

fn demo_integer_not_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.not_assign();
        println!("n := {}; n.not_assign(); n = {}", n_old, n);
    }
}

fn demo_integer_not(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

fn demo_integer_not_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

fn benchmark_integer_not_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.not_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.not_assign()))],
    );
}

fn benchmark_integer_not_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "!Integer",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(!n))),
            ("rug", &mut (|(n, _)| no_out!(!n))),
        ],
    );
}

fn benchmark_integer_not_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "!Integer",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("!Integer", &mut (|n| no_out!(!n))),
            ("!&Integer", &mut (|n| no_out!(!&n))),
        ],
    );
}
