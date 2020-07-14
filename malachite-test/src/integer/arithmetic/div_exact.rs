use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_integer_var_1, pairs_of_integer_and_nonzero_integer_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_exact_assign);
    register_demo!(registry, demo_integer_div_exact_assign_ref);
    register_demo!(registry, demo_integer_div_exact);
    register_demo!(registry, demo_integer_div_exact_val_ref);
    register_demo!(registry, demo_integer_div_exact_ref_val);
    register_demo!(registry, demo_integer_div_exact_ref_ref);
    register_bench!(
        registry,
        Small,
        benchmark_integer_div_exact_assign_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_integer_div_exact_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_integer_div_exact_library_comparison
    );
    register_bench!(registry, Small, benchmark_integer_div_exact_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_integer_div_exact_evaluation_strategy
    );
}

fn demo_integer_div_exact_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.div_exact_assign(y);
        println!("x := {}; x.div_exact_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_integer_div_exact_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer_var_1(gm).take(limit) {
        let x_old = x.clone();
        x.div_exact_assign(&y);
        println!("x := {}; x.div_exact_assign(&{}); x = {}", x_old, y, x);
    }
}

fn demo_integer_div_exact(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_exact({}) = {}", x_old, y_old, x.div_exact(y));
    }
}

fn demo_integer_div_exact_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.div_exact(&{}) = {}", x_old, y, x.div_exact(&y));
    }
}

fn demo_integer_div_exact_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_exact({}) = {}", x, y_old, (&x).div_exact(y));
    }
}

fn demo_integer_div_exact_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer_var_1(gm).take(limit) {
        println!("(&{}).div_exact(&{}) = {}", x, y, (&x).div_exact(&y));
    }
}

fn benchmark_integer_div_exact_assign_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact_assign(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(mut x, y)| x /= y)),
            ("exact division", &mut (|(mut x, y)| x.div_exact_assign(y))),
        ],
    );
}

fn benchmark_integer_div_exact_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_exact_assign(Integer)",
                &mut (|(mut x, y)| x.div_exact_assign(y)),
            ),
            (
                "Integer.div_exact_assign(&Integer)",
                &mut (|(mut x, y)| x.div_exact_assign(&y)),
            ),
        ],
    );
}

fn benchmark_integer_div_exact_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_integer_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_exact(y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_exact(&y)))),
        ],
    );
}

fn benchmark_integer_div_exact_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_exact(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(x, y)| no_out!(x / y))),
            ("exact division", &mut (|(x, y)| no_out!(x.div_exact(y)))),
        ],
    );
}

fn benchmark_integer_div_exact_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_exact(Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "Integer.div_exact(&Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(&y))),
            ),
            (
                "(&Integer).div_exact(Integer)",
                &mut (|(x, y)| no_out!((&x).div_exact(y))),
            ),
            (
                "(&Integer).div_exact(&Integer)",
                &mut (|(x, y)| no_out!((&x).div_exact(&y))),
            ),
        ],
    );
}
