use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(not(feature = "32_bit_limbs"))]
use inputs::natural::nm_pairs_of_natural_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{nrm_pairs_of_natural_and_unsigned, rm_pairs_of_unsigned_and_natural};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::Limb;
use natural::comparison::partial_ord_limb::num_partial_cmp_limb;
use num::BigUint;
use rug;
use std::cmp::Ordering;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_checked_sub_limb);
    register_demo!(registry, demo_natural_checked_sub_limb_ref);
    register_demo!(registry, demo_limb_checked_sub_natural);
    register_demo!(registry, demo_limb_checked_sub_natural_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_limb_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_sub_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_sub_natural_evaluation_strategy
    );
}

pub fn num_checked_sub_limb(x: BigUint, u: Limb) -> Option<BigUint> {
    if num_partial_cmp_limb(&x, u) != Some(Ordering::Less) {
        Some(x - BigUint::from(u))
    } else {
        None
    }
}

pub fn rug_checked_sub_u32(x: rug::Integer, u: u32) -> Option<rug::Integer> {
    if x >= u {
        Some(x - u)
    } else {
        None
    }
}

fn demo_natural_checked_sub_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.checked_sub({}) = {:?}", n_old, u, n.checked_sub(u));
    }
}

fn demo_natural_checked_sub_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).checked_sub({}) = {:?}", n, u, (&n).checked_sub(u));
    }
}

fn demo_limb_checked_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.checked_sub({}) = {:?}",
            u,
            n_old,
            CheckedSub::checked_sub(u, n)
        );
    }
}

fn demo_limb_checked_sub_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.checked_sub(&{}) = {:?}",
            u,
            n_old,
            CheckedSub::checked_sub(u, &n)
        );
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_checked_sub_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.checked_sub(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_checked_sub_limb(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_checked_sub_u32(x, y))),
            ),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_natural_checked_sub_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.checked_sub(y)))),
            (
                "num",
                &mut (|((x, y), _)| no_out!(num_checked_sub_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.checked_sub(Limb)",
                &mut (|(x, y)| no_out!(x.checked_sub(y))),
            ),
            (
                "(&Natural).checked_sub(Limb)",
                &mut (|(x, y)| no_out!((&x).checked_sub(y))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_checked_sub_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.checked_sub(Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(CheckedSub::checked_sub(x, y))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_limb_checked_sub_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.checked_sub(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.checked_sub(Natural)",
                &mut (|(x, y)| no_out!(CheckedSub::checked_sub(x, y))),
            ),
            (
                "Limb.checked_sub(&Natural)",
                &mut (|(x, y)| no_out!(CheckedSub::checked_sub(x, &y))),
            ),
        ],
    );
}
