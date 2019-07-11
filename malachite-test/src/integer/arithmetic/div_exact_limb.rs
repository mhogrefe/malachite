use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_limb_var_1, pairs_of_integer_and_positive_limb_var_1,
    pairs_of_limb_and_nonzero_integer_var_2,
};
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::Limb;
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_exact_assign_limb);
    register_demo!(registry, demo_integer_div_exact_limb);
    register_demo!(registry, demo_integer_div_exact_limb_ref);
    register_demo!(registry, demo_limb_div_exact_integer);
    register_demo!(registry, demo_limb_div_exact_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_assign_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_exact_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_limb_div_exact_integer_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_exact_integer_evaluation_strategy
    );
}

pub fn rug_div_exact_limb(x: rug::Integer, u: Limb) -> rug::Integer {
    x.div_exact(&rug::Integer::from(u))
}

fn demo_integer_div_exact_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_limb_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.div_exact_assign(u);
        println!("x := {}; x.div_exact_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_div_exact_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_limb_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", n_old, u, n.div_exact(u));
    }
}

fn demo_integer_div_exact_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_limb_var_1(gm).take(limit) {
        println!("(&{}).div_exact({}) = {}", n, u, (&n).div_exact(u));
    }
}

fn demo_limb_div_exact_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_limb_and_nonzero_integer_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", u, n_old, u.div_exact(n));
    }
}

fn demo_limb_div_exact_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_limb_and_nonzero_integer_var_2(gm).take(limit) {
        println!("{}.div_exact(&{}) = {}", u, n, u.div_exact(&n));
    }
}

fn benchmark_integer_div_exact_assign_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact_assign(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(mut x, y)| x /= y)),
            ("exact division", &mut (|(mut x, y)| x.div_exact_assign(y))),
        ],
    );
}

fn benchmark_integer_div_exact_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_exact(y)))),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_exact_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_exact_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_exact(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(ref x, y)| no_out!(x / y))),
            (
                "exact division",
                &mut (|(ref x, y)| no_out!(x.div_exact(y))),
            ),
        ],
    );
}

fn benchmark_integer_div_exact_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_exact(Limb)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "(&Integer).div_exact(Limb)",
                &mut (|(x, y)| no_out!((&x).div_exact(y))),
            ),
        ],
    );
}

fn benchmark_limb_div_exact_integer_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Limb.div_exact(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_limb_and_nonzero_integer_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(x, y)| no_out!(x / y))),
            ("exact division", &mut (|(x, y)| no_out!(x.div_exact(y)))),
        ],
    );
}

fn benchmark_limb_div_exact_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_limb_and_nonzero_integer_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_exact(Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "Limb.div_exact(&Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(&y))),
            ),
        ],
    );
}
