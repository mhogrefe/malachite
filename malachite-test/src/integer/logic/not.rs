use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{integers, rm_integers};
use malachite_base::num::SignificantBits;
use malachite_base::num::NotAssign;

pub fn demo_integer_not_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.not_assign();
        println!("n := {}; n.not_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_not(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

pub fn demo_integer_not_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

pub fn benchmark_integer_not_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.not_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|mut n| n.not_assign()))],
    );
}

pub fn benchmark_integer_not_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "!Integer",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, n)| no_out!(!n))),
            ("rug", &mut (|(n, _)| no_out!(!n))),
        ],
    );
}

pub fn benchmark_integer_not_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "!Integer",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("!Integer", &mut (|n| no_out!(!n))),
            ("!&Integer", &mut (|n| no_out!(!&n))),
        ],
    );
}
