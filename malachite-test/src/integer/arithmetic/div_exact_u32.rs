use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_u32_var_1, pairs_of_integer_and_positive_u32_var_1,
    pairs_of_u32_and_nonzero_integer_var_2,
};
use malachite_base::num::{DivExact, DivExactAssign, SignificantBits};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_exact_assign_u32);
    register_demo!(registry, demo_integer_div_exact_u32);
    register_demo!(registry, demo_integer_div_exact_u32_ref);
    register_demo!(registry, demo_u32_div_exact_integer);
    register_demo!(registry, demo_u32_div_exact_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_assign_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_exact_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_u32_div_exact_integer_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_exact_integer_evaluation_strategy
    );
}

pub fn rug_div_exact_u32(x: rug::Integer, u: u32) -> rug::Integer {
    x.div_exact(&rug::Integer::from(u))
}

fn demo_integer_div_exact_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_u32_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.div_exact_assign(u);
        println!("x := {}; x.div_exact_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_div_exact_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_u32_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", n_old, u, n.div_exact(u));
    }
}

fn demo_integer_div_exact_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_u32_var_1(gm).take(limit) {
        println!("(&{}).div_exact({}) = {}", n, u, (&n).div_exact(u));
    }
}

fn demo_u32_div_exact_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_u32_and_nonzero_integer_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", u, n_old, u.div_exact(n));
    }
}

fn demo_u32_div_exact_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_u32_and_nonzero_integer_var_2(gm).take(limit) {
        println!("{}.div_exact(&{}) = {}", u, n, u.div_exact(&n));
    }
}

fn benchmark_integer_div_exact_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact_assign(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(mut x, y)| x /= y)),
            ("exact division", &mut (|(mut x, y)| x.div_exact_assign(y))),
        ],
    );
}

fn benchmark_integer_div_exact_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_exact(y)))),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_exact_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_exact_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_exact(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
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

fn benchmark_integer_div_exact_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_exact(u32)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "(&Integer).div_exact(u32)",
                &mut (|(x, y)| no_out!((&x).div_exact(y))),
            ),
        ],
    );
}

fn benchmark_u32_div_exact_integer_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32.div_exact(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_u32_and_nonzero_integer_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(x, y)| no_out!(x / y))),
            ("exact division", &mut (|(x, y)| no_out!(x.div_exact(y)))),
        ],
    );
}

fn benchmark_u32_div_exact_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_u32_and_nonzero_integer_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_exact(Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "u32.div_exact(&Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(&y))),
            ),
        ],
    );
}
