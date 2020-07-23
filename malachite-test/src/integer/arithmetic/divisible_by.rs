use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz_test_util::integer::arithmetic::divisible_by::num_divisible_by;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{nrm_pairs_of_integers, pairs_of_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by);
    register_demo!(registry, demo_integer_divisible_by_val_ref);
    register_demo!(registry, demo_integer_divisible_by_ref_val);
    register_demo!(registry, demo_integer_divisible_by_ref_ref);
    register_bench!(registry, Large, benchmark_integer_divisible_by_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_library_comparison
    );
}

fn demo_integer_divisible_by(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.divisible_by(y) {
            println!("{} is divisible by {}", x_old, y_old);
        } else {
            println!("{} is not divisible by {}", x_old, y_old);
        }
    }
}

fn demo_integer_divisible_by_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        if x.divisible_by(&y) {
            println!("{} is divisible by {}", x_old, y);
        } else {
            println!("{} is not divisible by {}", x_old, y);
        }
    }
}

fn demo_integer_divisible_by_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        if (&x).divisible_by(y) {
            println!("{} is divisible by {}", x, y_old);
        } else {
            println!("{} is not divisible by {}", x, y_old);
        }
    }
}

fn demo_integer_divisible_by_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        if (&x).divisible_by(&y) {
            println!("{} is divisible by {}", x, y);
        } else {
            println!("{} is not divisible by {}", x, y);
        }
    }
}

fn benchmark_integer_divisible_by_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.divisible_by(y)))),
            (
                "using %",
                &mut (|(x, y)| no_out!(x == 0 || y != 0 && x % y == 0)),
            ),
        ],
    );
}

fn benchmark_integer_divisible_by_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Integer.divisible_by(Integer)",
                &mut (|(x, y)| no_out!(x.divisible_by(y))),
            ),
            (
                "Integer.divisible_by(&Integer)",
                &mut (|(x, y)| no_out!(x.divisible_by(&y))),
            ),
            (
                "(&Integer).divisible_by(Integer)",
                &mut (|(x, y)| no_out!((&x).divisible_by(y))),
            ),
            (
                "(&Integer).divisible_by(&Integer)",
                &mut (|(x, y)| no_out!((&x).divisible_by(&y))),
            ),
        ],
    );
}

fn benchmark_integer_divisible_by_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, _))| usize::exact_from(x.significant_bits())),
        "y.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.divisible_by(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_divisible_by(&x, &y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.is_divisible(&y)))),
        ],
    );
}
