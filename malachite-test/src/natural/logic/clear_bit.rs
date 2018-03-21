use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::pairs_of_natural_and_small_u64;
use malachite_base::num::BitAccess;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_clear_bit);
    register_bench!(registry, Large, benchmark_natural_clear_bit);
}

fn demo_natural_clear_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_natural_clear_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.clear_bit(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [("malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}
