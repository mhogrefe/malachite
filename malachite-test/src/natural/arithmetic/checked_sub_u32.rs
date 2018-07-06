use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
    pairs_of_unsigned_and_natural, rm_pairs_of_unsigned_and_natural,
};
use malachite_base::num::{CheckedSub, SignificantBits};
use natural::comparison::partial_ord_u32::num_partial_cmp_u32;
use num::BigUint;
use rug;
use std::cmp::Ordering;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_checked_sub_u32);
    register_demo!(registry, demo_natural_checked_sub_u32_ref);
    register_demo!(registry, demo_u32_checked_sub_natural);
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_checked_sub_natural_library_comparison
    );
}

pub fn num_checked_sub_u32(x: BigUint, u: u32) -> Option<BigUint> {
    if num_partial_cmp_u32(&x, u) != Some(Ordering::Less) {
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

fn demo_natural_checked_sub_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.checked_sub({}) = {:?}", n_old, u, n.checked_sub(u));
    }
}

fn demo_natural_checked_sub_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).checked_sub({}) = {:?}", n, u, (&n).checked_sub(u));
    }
}

fn demo_u32_checked_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.checked_sub({}) = {:?}",
            u,
            n_old,
            CheckedSub::checked_sub(u, &n)
        );
    }
}

fn benchmark_natural_checked_sub_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.checked_sub(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_checked_sub_u32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_checked_sub_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.checked_sub(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.checked_sub(u32)",
                &mut (|(x, y)| no_out!(x.checked_sub(y))),
            ),
            (
                "(&Natural).checked_sub(u32)",
                &mut (|(x, y)| no_out!((&x).checked_sub(y))),
            ),
        ],
    );
}

fn benchmark_u32_checked_sub_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.checked_sub(&Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(CheckedSub::checked_sub(x, &y))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}
