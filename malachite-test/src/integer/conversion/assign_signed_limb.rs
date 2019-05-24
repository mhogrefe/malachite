use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{Assign, SignificantBits};
use malachite_nz::platform::SignedLimb;
use num::BigInt;
use rug::Assign as rug_assign;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_assign_signed_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_signed_limb_library_comparison
    );
}

pub fn num_assign_signed_limb(x: &mut BigInt, i: SignedLimb) {
    *x = BigInt::from(i);
}

fn demo_integer_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

fn benchmark_integer_assign_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (mut x, y))| x.assign(y))),
            (
                "num",
                &mut (|((mut x, y), _, _)| num_assign_signed_limb(&mut x, y)),
            ),
            ("rug", &mut (|(_, (mut x, y), _)| x.assign(y))),
        ],
    );
}
