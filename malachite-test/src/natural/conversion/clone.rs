use std::cmp::max;

use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{naturals, nrm_naturals, nrm_pairs_of_naturals, pairs_of_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_clone);
    register_demo!(registry, demo_natural_clone_from);
    register_bench!(registry, Large, benchmark_natural_clone_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_natural_clone_from_library_comparison
    );
}

fn demo_natural_clone(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

fn demo_natural_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

fn benchmark_natural_clone_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.clone()",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.clone()))),
            ("num", &mut (|(n, _, _)| no_out!(n.clone()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.clone()))),
        ],
    );
}

fn benchmark_natural_clone_from_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.clone_from(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (mut x, y))| x.clone_from(&y))),
            ("num", &mut (|((mut x, y), _, _)| x.clone_from(&y))),
            ("rug", &mut (|(_, (mut x, y), _)| x.clone_from(&y))),
        ],
    );
}
