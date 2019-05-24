use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(feature = "64_bit_limbs")]
use inputs::integer::nm_pairs_of_integer_and_signed;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::{
    nrm_pairs_of_integer_and_signed, rm_pairs_of_integer_and_signed, rm_pairs_of_signed_and_integer,
};
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::SignedLimb;
use num::BigInt;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_sub_assign_signed_limb);
    register_demo!(registry, demo_integer_sub_signed_limb);
    register_demo!(registry, demo_integer_sub_signed_limb_ref);
    register_demo!(registry, demo_signed_limb_sub_integer);
    register_demo!(registry, demo_signed_limb_sub_integer_ref);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_assign_signed_limb_library_comparison
    );
    #[cfg(feature = "64_bit_limbs")]
    register_bench!(registry, Large, benchmark_integer_sub_assign_signed_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_signed_limb_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_sub_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_sub_integer_evaluation_strategy
    );
}

pub fn num_sub_signed_limb(x: BigInt, i: SignedLimb) -> BigInt {
    x - BigInt::from(i)
}

fn demo_integer_sub_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n -= i;
        println!("x := {}; x -= {}; x = {}", n_old, i, n);
    }
}

fn demo_integer_sub_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, i, n - i);
    }
}

fn demo_integer_sub_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        println!("&{} - {} = {}", n, i, &n - i);
    }
}

fn demo_signed_limb_sub_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", i, n_old, i - n);
    }
}

fn demo_signed_limb_sub_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - &{} = {}", i, n_old, i - &n);
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_sub_assign_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer -= SignedLimb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_sub_assign_signed_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer -= SignedLimb",
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x -= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_sub_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - SignedLimb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_sub_signed_limb(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_sub_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - SignedLimb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            (
                "num",
                &mut (|((x, y), _)| no_out!(num_sub_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_sub_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - SignedLimb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Integer - SignedLimb", &mut (|(x, y)| no_out!(x - y))),
            ("&Integer - SignedLimb", &mut (|(x, y)| no_out!(&x - y))),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_signed_limb_sub_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb - Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x - y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_signed_limb_sub_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb - Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("SignedLimb - Integer", &mut (|(x, y)| no_out!(x - y))),
            ("SignedLimb - &Integer", &mut (|(x, y)| no_out!(x - &y))),
        ],
    );
}
