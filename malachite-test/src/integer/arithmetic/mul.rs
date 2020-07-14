use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{
    nrm_pairs_of_integers, pairs_of_integers, rm_pairs_of_integers,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_mul_assign);
    register_demo!(registry, demo_integer_mul_assign_ref);
    register_demo!(registry, demo_integer_mul);
    register_demo!(registry, demo_integer_mul_val_ref);
    register_demo!(registry, demo_integer_mul_ref_val);
    register_demo!(registry, demo_integer_mul_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mul_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_mul_library_comparison);
    register_bench!(registry, Large, benchmark_integer_mul_evaluation_strategy);
}

fn demo_integer_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_mul(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

fn demo_integer_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

fn demo_integer_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

fn demo_integer_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

fn benchmark_integer_mul_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer *= Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

fn benchmark_integer_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer *= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Integer *= Integer", &mut (|(mut x, y)| no_out!(x *= y))),
            ("Integer *= &Integer", &mut (|(mut x, y)| no_out!(x *= &y))),
        ],
    );
}

fn benchmark_integer_mul_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer * Integer",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x * y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_integer_mul_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer * Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(x.significant_bits() + y.significant_bits())),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Integer * Integer", &mut (|(x, y)| no_out!(x * y))),
            ("Integer * &Integer", &mut (|(x, y)| no_out!(x * &y))),
            ("&Integer * Integer", &mut (|(x, y)| no_out!(&x * y))),
            ("&Integer * &Integer", &mut (|(x, y)| no_out!(&x * &y))),
        ],
    );
}
