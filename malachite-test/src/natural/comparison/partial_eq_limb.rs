use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
    pairs_of_unsigned_and_natural, rm_pairs_of_unsigned_and_natural,
};
use malachite_base::num::SignificantBits;
use malachite_nz::platform::Limb;
use num::BigUint;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_partial_eq_limb);
    register_demo!(registry, demo_limb_partial_eq_natural);
    register_bench!(
        registry,
        Large,
        benchmark_natural_partial_eq_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_partial_eq_natural_library_comparison
    );
}

pub fn num_partial_eq_limb(x: &BigUint, u: Limb) -> bool {
    *x == BigUint::from(u)
}

fn demo_natural_partial_eq_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

fn demo_limb_partial_eq_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

fn benchmark_natural_partial_eq_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural == Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
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

fn benchmark_limb_partial_eq_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb == Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x == y))),
            ("rug", &mut (|((x, y), _)| no_out!(x == y))),
        ],
    );
}
