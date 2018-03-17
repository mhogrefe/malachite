use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
                      pairs_of_unsigned_and_natural, rm_pairs_of_unsigned_and_natural,
                      pairs_of_natural_and_u32_var_1, rm_pairs_of_natural_and_u32_var_1};
use malachite_base::num::SignificantBits;
use natural::comparison::partial_ord_u32::num_partial_cmp_u32;
use num::BigUint;
use rug;
use std::cmp::Ordering;

pub fn num_sub_u32(x: BigUint, u: u32) -> Option<BigUint> {
    if num_partial_cmp_u32(&x, u) != Some(Ordering::Less) {
        Some(x - BigUint::from(u))
    } else {
        None
    }
}

pub fn rug_sub_u32(x: rug::Integer, u: u32) -> Option<rug::Integer> {
    if x >= u {
        Some(x - u)
    } else {
        None
    }
}

pub fn demo_natural_sub_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_u32_var_1(gm).take(limit) {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_natural_sub_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {:?}", n_old, u, n - u);
    }
}

pub fn demo_natural_sub_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} - {} = {:?}", n, u, &n - u);
    }
}

pub fn demo_u32_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {:?}", u, n_old, u - &n);
    }
}

pub fn benchmark_natural_sub_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer -= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

pub fn benchmark_natural_sub_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_sub_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

pub fn benchmark_natural_sub_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("Integer - u32", &mut (|(x, y)| no_out!(x - y))),
            ("&Integer - u32", &mut (|(x, y)| no_out!(&x - y))),
        ],
    );
}

pub fn benchmark_u32_sub_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 - Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, (x, y))| no_out!(x - &y))),
            ("rug", &mut (|((x, y), _)| no_out!(x - y))),
        ],
    );
}
