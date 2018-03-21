use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nrm_pairs_of_natural_and_small_u64, pairs_of_natural_and_small_u64};
use malachite_base::num::BitAccess;
use num::{BigUint, One, Zero};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_get_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_get_bit_library_comparison
    );
}

pub fn num_get_bit(x: &BigUint, index: u64) -> bool {
    x & (BigUint::one() << index as usize) != BigUint::zero()
}

fn demo_natural_get_bit(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_natural_and_small_u64(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_natural_get_bit_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.get_bit(u64)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (_, index))| index as usize),
        "index",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (n, index))| no_out!(n.get_bit(index))),
            ),
            (
                "num",
                &mut (|((n, index), _, _)| no_out!(num_get_bit(&n, index))),
            ),
            (
                "rug",
                &mut (|(_, (n, index), _)| no_out!(n.get_bit(index as u32))),
            ),
        ],
    );
}
