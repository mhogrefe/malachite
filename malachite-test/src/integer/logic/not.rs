use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{integers, rm_integers};

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
    run_benchmark(
        "Integer.not_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.not_assign()))],
    );
}

fn benchmark_integer_not_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "!Integer",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(!n))),
            ("rug", &mut (|(n, _)| no_out!(!n))),
        ],
    );
}

fn benchmark_integer_not_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "!Integer",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("!Integer", &mut (|n| no_out!(!n))),
            ("!&Integer", &mut (|n| no_out!(!&n))),
        ],
    );
}
