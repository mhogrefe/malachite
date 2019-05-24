use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_signed_limb_var_1,
    pairs_of_integer_and_nonzero_signed_limb_var_1, pairs_of_signed_limb_and_nonzero_integer_var_2,
};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{DivExact, DivExactAssign, SignificantBits};
use malachite_nz::platform::SignedLimb;
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_exact_assign_signed_limb);
    register_demo!(registry, demo_integer_div_exact_signed_limb);
    register_demo!(registry, demo_integer_div_exact_signed_limb_ref);
    register_demo!(registry, demo_signed_limb_div_exact_integer);
    register_demo!(registry, demo_signed_limb_div_exact_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_assign_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_exact_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_div_exact_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_div_exact_integer_evaluation_strategy
    );
}

pub fn rug_div_exact_signed_limb(x: rug::Integer, i: SignedLimb) -> rug::Integer {
    x.div_exact(&rug::Integer::from(i))
}

fn demo_integer_div_exact_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed_limb_var_1::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n.div_exact_assign(i);
        println!("x := {}; x.div_exact_assign({}); x = {}", n_old, i, n);
    }
}

fn demo_integer_div_exact_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed_limb_var_1::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", n_old, i, n.div_exact(i));
    }
}

fn demo_integer_div_exact_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed_limb_var_1::<SignedLimb>(gm).take(limit) {
        println!("(&{}).div_exact({}) = {}", n, i, (&n).div_exact(i));
    }
}

fn demo_signed_limb_div_exact_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_limb_and_nonzero_integer_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", i, n_old, i.div_exact(n));
    }
}

fn demo_signed_limb_div_exact_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_limb_and_nonzero_integer_var_2(gm).take(limit) {
        println!("{}.div_exact(&{}) = {}", i, n, i.div_exact(&n));
    }
}

fn benchmark_integer_div_exact_assign_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact_assign(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed_limb_var_1::<SignedLimb>(gm),
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

fn benchmark_integer_div_exact_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed_limb_var_1(gm),
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
                &mut (|(_, (x, y), _)| no_out!(rug_div_exact_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_exact_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed_limb_var_1::<SignedLimb>(gm),
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

fn benchmark_integer_div_exact_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_exact(SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed_limb_var_1::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_exact(SignedLimb)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "(&Integer).div_exact(SignedLimb)",
                &mut (|(x, y)| no_out!((&x).div_exact(y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_div_exact_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.div_exact(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_signed_limb_and_nonzero_integer_var_2(gm),
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

fn benchmark_signed_limb_div_exact_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_limb_and_nonzero_integer_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.div_exact(Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "SignedLimb.div_exact(&Integer)",
                &mut (|(x, y)| no_out!(x.div_exact(&y))),
            ),
        ],
    );
}
