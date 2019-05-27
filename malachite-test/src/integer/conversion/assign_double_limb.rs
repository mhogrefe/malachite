use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{Assign, SignificantBits};
use malachite_nz::platform::DoubleLimb;
use num::BigInt;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{nm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_assign_double_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_double_limb_library_comparison
    );
}

pub fn num_assign_double_limb(x: &mut BigInt, u: DoubleLimb) {
    *x = BigInt::from(u);
}

fn demo_integer_assign_double_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<DoubleLimb>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

fn benchmark_integer_assign_double_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(DoubleLimb)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_unsigned(gm),
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
