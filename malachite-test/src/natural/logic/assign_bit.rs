use malachite_base::num::traits::BitAccess;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    rm_triples_of_natural_small_u64_and_bool, triples_of_natural_small_u64_and_bool,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_assign_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_bit_library_comparison
    );
}

fn demo_natural_assign_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index, bit) in triples_of_natural_small_u64_and_bool(gm).take(limit) {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn benchmark_natural_assign_bit_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.assign_bit(u64, bool)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_natural_small_u64_and_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index, _))| index as usize),
        "index",
        &mut [
            (
                "malachite",
                &mut (|(_, (mut n, index, bit))| n.assign_bit(index, bit)),
            ),
            (
                "rug",
                &mut (|((mut n, index, bit), _)| no_out!(n.set_bit(index as u32, bit))),
            ),
        ],
    );
}
