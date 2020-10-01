use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz_test_util::natural::arithmetic::neg::neg_num;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{naturals, nrm_naturals};

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

fn benchmark_natural_neg_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "-Natural",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, n)| no_out!(-n))),
            ("num", &mut (|(n, _, _)| no_out!(neg_num(n)))),
            ("rug", &mut (|(_, n, _)| no_out!(-n))),
        ],
    );
}

fn benchmark_natural_neg_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "-Natural",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("-Natural", &mut (|n| no_out!(-n))),
            ("-&Natural", &mut (|n| no_out!(-&n))),
        ],
    );
}
