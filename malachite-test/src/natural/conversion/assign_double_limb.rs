use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{Assign, SignificantBits};
use malachite_nz::platform::DoubleLimb;
use num::BigUint;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_assign_double_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_double_limb_library_comparison
    );
}

pub fn num_assign_double_limb(x: &mut BigUint, u: DoubleLimb) {
    *x = BigUint::from(u);
}

fn demo_natural_assign_double_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<DoubleLimb>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

fn benchmark_natural_assign_double_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.assign(DoubleLimb)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            (
                "num",
                &mut (|((mut x, y), _)| num_assign_double_limb(&mut x, y)),
            ),
        ],
    );
}
