use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_square_assign);
    register_demo!(registry, demo_integer_square);
    register_demo!(registry, demo_integer_square_ref);
    register_bench!(registry, Large, benchmark_integer_square_assign);
    register_bench!(registry, Large, benchmark_integer_square_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_square_evaluation_strategy
    );
}

fn demo_integer_square_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let old_n = n.clone();
        n.square_assign();
        println!("n := {}; n.square_assign(); n = {}", n, old_n);
    }
}

fn demo_integer_square(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("({}) ^ 2 = {}", n.clone(), n.square());
    }
}

fn demo_integer_square_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("&{} ^ 2 = {}", n, (&n).square());
    }
}

fn benchmark_integer_square_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.square_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|mut n| n.square_assign()))],
    );
}

fn benchmark_integer_square_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.square()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|ref n| no_out!(n.square()))),
            ("using *", &mut (|ref n| no_out!(n * n))),
        ],
    );
}

fn benchmark_integer_square_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.square()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Integer.square()", &mut (|n| no_out!(n.square()))),
            ("(&Integer).square()", &mut (|n| no_out!((&n).square()))),
        ],
    );
}
