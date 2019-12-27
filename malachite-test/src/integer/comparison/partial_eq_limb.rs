use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::Limb;
use num::BigInt;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned,
    pairs_of_unsigned_and_integer, rm_pairs_of_unsigned_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_eq_limb);
    register_demo!(registry, demo_limb_partial_eq_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_eq_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_partial_eq_integer_library_comparison
    );
}

pub fn num_partial_eq_limb(x: &BigInt, u: Limb) -> bool {
    *x == BigInt::from(u)
}

fn demo_integer_partial_eq_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

fn demo_limb_partial_eq_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

fn benchmark_integer_partial_eq_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer == Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x == y))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_eq_limb(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x == y))),
        ],
    );
}

fn benchmark_limb_partial_eq_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb == Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}
