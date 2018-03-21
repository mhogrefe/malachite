use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::pairs_of_integer_and_small_u64;
use malachite_base::num::{BitAccess, SignificantBits};
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_clear_bit);
    register_bench!(registry, Large, benchmark_integer_clear_bit);
}

fn demo_integer_clear_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_integer_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_integer_clear_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.clear_bit(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, index)| max(n.significant_bits(), index) as usize),
        "max(n.significant_bits(), index)",
        &mut [("malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}
