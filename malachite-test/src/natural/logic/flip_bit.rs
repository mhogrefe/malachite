use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    pairs_of_natural_and_small_unsigned, rm_pairs_of_natural_and_small_unsigned,
};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::BitAccess;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_flip_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_flip_bit_library_comparison
    );
}

fn demo_natural_flip_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_natural_flip_bit_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.flip_bit(u64)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index))| usize::checked_from(index).unwrap()),
        "n.significant_bits()",
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
