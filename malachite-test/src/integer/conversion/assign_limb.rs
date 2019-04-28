use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{nrm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned};
use malachite_base::num::traits::{Assign, SignificantBits};
use malachite_nz::platform::Limb;
use num::BigInt;
use rug::Assign as rug_assign;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_limb_library_comparison
    );
}

pub fn num_assign_limb(x: &mut BigInt, u: Limb) {
    *x = BigInt::from(u);
}

fn demo_integer_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

fn benchmark_integer_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (mut x, y))| x.assign(y))),
            (
                "num",
                &mut (|((mut x, y), _, _)| num_assign_limb(&mut x, y)),
            ),
            ("rug", &mut (|(_, (mut x, y), _)| x.assign(y))),
        ],
    );
}
