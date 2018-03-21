use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nm_pairs_of_natural_and_small_u64, pairs_of_natural_and_small_u64};
use malachite_base::num::BitAccess;
use num::{BigUint, One};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_set_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_set_bit_library_comparison
    );
}

pub fn num_set_bit(x: &mut BigUint, index: u64) {
    *x = x.clone() | (BigUint::one() << index as usize);
}

fn demo_natural_set_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_natural_set_bit_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.set_bit(u64)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index))| index as usize),
        "index",
        &mut [
            ("malachite", &mut (|(_, (mut n, index))| n.set_bit(index))),
            (
                "num",
                &mut (|((mut n, index), _)| num_set_bit(&mut n, index)),
            ),
        ],
    );
}
