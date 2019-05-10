use std::cmp::max;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{BitAccess, SignificantBits};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_small_u64, rm_pairs_of_integer_and_small_u64};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_flip_bit);
    register_bench!(
        registry,
        Large,
        benchmark_integer_flip_bit_library_comparison
    );
}

fn demo_integer_flip_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_integer_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_integer_flip_bit_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.flip_bit(u64)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, index))| usize::checked_from(max(n.significant_bits(), index)).unwrap()),
        "max(n.significant_bits(), index)",
        &mut [
            ("malachite", &mut (|(_, (mut n, index))| n.flip_bit(index))),
            (
                "rug",
                &mut (|((mut n, index), _)| {
                    no_out!(n.toggle_bit(u32::checked_from(index).unwrap()))
                }),
            ),
        ],
    );
}
