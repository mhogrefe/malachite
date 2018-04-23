use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned};
use malachite_base::num::Assign;
use malachite_base::num::SignificantBits;
use num::BigUint;
use rug::Assign as rug_assign;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_u32_library_comparison
    );
}

pub fn num_assign_u32(x: &mut BigUint, u: u32) {
    *x = BigUint::from(u);
}

fn demo_natural_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

fn benchmark_natural_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.assign(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (mut x, y))| x.assign(y))),
            ("num", &mut (|((mut x, y), _, _)| num_assign_u32(&mut x, y))),
            ("rug", &mut (|(_, (mut x, y), _)| x.assign(y))),
        ],
    );
}
