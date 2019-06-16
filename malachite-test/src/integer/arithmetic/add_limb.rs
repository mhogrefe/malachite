use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::Limb;
use num::BigInt;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(not(feature = "32_bit_limbs"))]
use inputs::integer::nm_pairs_of_integer_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::{
    nrm_pairs_of_integer_and_unsigned, rm_pairs_of_integer_and_unsigned,
    rm_pairs_of_unsigned_and_integer,
};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_add_assign_limb);
    register_demo!(registry, demo_integer_add_limb);
    register_demo!(registry, demo_integer_add_limb_ref);
    register_demo!(registry, demo_limb_add_integer);
    register_demo!(registry, demo_limb_add_integer_ref);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_assign_limb_library_comparison
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    register_bench!(registry, Large, benchmark_integer_add_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_limb_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_add_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_add_integer_evaluation_strategy
    );
}

pub fn num_add_limb(x: BigInt, u: Limb) -> BigInt {
    x + BigInt::from(u)
}

fn demo_integer_add_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n += u;
        println!("x := {}; x += {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_add_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", n_old, u, n + u);
    }
}

fn demo_integer_add_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        println!("&{} + {} = {}", n, u, &n + u);
    }
}

fn demo_limb_add_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", u, n_old, u + n);
    }
}

fn demo_limb_add_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + &{} = {}", u, n_old, u + &n);
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_add_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer += Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x += y)),
            ("rug", &mut (|((mut x, y), _)| x += y)),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_integer_add_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer += Limb",
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x += y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_add_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer + Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x + y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_add_limb(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x + y))),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_integer_add_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer + Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x + y))),
            ("num", &mut (|((x, y), _)| no_out!(num_add_limb(x, y)))),
        ],
    );
}

fn benchmark_integer_add_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer + Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Integer + Limb", &mut (|(x, y)| no_out!(x + y))),
            ("&Integer + Limb", &mut (|(x, y)| no_out!(&x + y))),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_add_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb + Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x + y))),
            ("rug", &mut (|((x, y), _)| no_out!(x + y))),
        ],
    );
}

fn benchmark_limb_add_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb + Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb + Integer", &mut (|(x, y)| no_out!(x + y))),
            ("Limb + &Integer", &mut (|(x, y)| no_out!(x + &y))),
        ],
    );
}
