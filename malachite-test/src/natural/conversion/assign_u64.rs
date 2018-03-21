use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use num::BigUint;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_assign_u64);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_u64_library_comparison
    );
}

pub fn num_assign_u64(x: &mut BigUint, u: u64) {
    *x = BigUint::from(u);
}

fn demo_natural_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

fn benchmark_natural_assign_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.assign(u64)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("num", &mut (|((mut x, y), _)| num_assign_u64(&mut x, y))),
        ],
    );
}
